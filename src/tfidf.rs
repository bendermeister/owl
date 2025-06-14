use crate::file::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use stemmer::Stemmer;

fn term_histogram_plain_text(file: &impl FileLike) -> HashMap<String, u64> {
    let body = file.read();
    let mut stemmer = Stemmer::new("english").unwrap();
    let terms = body.split_whitespace().map(|s| stemmer.stem(s));

    let mut histogram = HashMap::new();

    for term in terms {
        *histogram.entry(term).or_insert(0) += 1;
    }

    histogram
}

pub fn term_histogram(file: &impl FileLike) -> HashMap<String, u64> {
    match file.file_format() {
        crate::file_format::FileFormat::Unknown => HashMap::new(),
        crate::file_format::FileFormat::Markdown => term_histogram_plain_text(file),
        crate::file_format::FileFormat::Typst => term_histogram_plain_text(file),
    }
}

fn get_tf(db: &rusqlite::Connection, file: i64, term: i64) -> Result<f64, anyhow::Error> {
    let mut stmt =
        db.prepare_cached("SELECT tf FROM term_frequencies WHERE term = ? AND file = ? LIMIT 1;")?;
    let row: Result<f64, _> = stmt.query_one(rusqlite::params![term, file], |row| row.get(0));

    let tf = match row {
        Ok(row) => row,
        Err(rusqlite::Error::QueryReturnedNoRows) => 0 as f64,
        Err(e) => return Err(anyhow::anyhow!("failed to fetch tf from db: {:?}", e)),
    };

    Ok(tf)
}

pub fn rank(db: &rusqlite::Connection, phrase: &str) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut stemmer = Stemmer::new("english").unwrap();
    let mut phrase: Vec<_> = phrase.split_whitespace().map(|s| stemmer.stem(s)).collect();
    phrase.sort();

    let files: HashMap<i64, PathBuf> = db
        .prepare("SELECT id, path FROM files;")?
        .query([])?
        .and_then(|row| Ok((row.get(0)?, row.get::<_, String>(1)?.into())))
        .collect::<Result<_, anyhow::Error>>()?;

    let mut terms = Vec::new();
    for term in phrase.iter() {
        let id: i64 = db
            .prepare_cached("SELECT id FROM terms WHERE term = ?")?
            .query_row([term], |row| row.get(0))?;
        terms.push(id);
    }

    let mut idf: HashMap<i64, f64> = HashMap::new();
    let file_count = files.len() as f64;

    for term in terms.iter() {
        let term_count: i64 = db
            .prepare_cached(
                "SELECT COUNT(*) FROM (SELECT DISTINCT file FROM term_frequencies WHERE term = ?);",
            )?
            .query_row([term], |row| row.get(0))?;
        let term_count = term_count as f64;
        idf.insert(*term, file_count / term_count);
    }

    let mut tf_idf: HashMap<i64, f64> = files.keys().map(|id| (*id, 0 as f64)).collect();

    for term in terms.iter() {
        for file in files.keys() {
            let tf = get_tf(db, *file, *term)?;
            *tf_idf.get_mut(file).unwrap() += tf * idf.get(term).unwrap();
        }
    }

    let mut ranking: Vec<i64> = files.keys().copied().collect();
    ranking.sort_by(|a, b| {
        tf_idf
            .get(a)
            .unwrap()
            .partial_cmp(tf_idf.get(b).unwrap())
            .unwrap()
            .reverse()
    });

    let ranking: Vec<PathBuf> = ranking
        .iter()
        .map(|id| files.get(id).unwrap().clone())
        .collect();

    Ok(ranking)
}
