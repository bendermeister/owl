use crate::file_format::FileFormat;
use crate::stemmer;
use crate::time_stamp::TimeStamp;
use crate::todo;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

struct File {
    id: i64,
    path: String,
    last_touched: TimeStamp,
}

fn discover_paths(dir: fs::ReadDir) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut buf = Vec::new();

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let mut sub_dir = discover_paths(fs::read_dir(path)?)?;
            buf.append(&mut sub_dir);
        } else {
            buf.push(path);
        }
    }

    Ok(buf)
}

fn discover_files(dir: fs::ReadDir) -> Result<Vec<File>, anyhow::Error> {
    let paths = discover_paths(dir)?;
    paths
        .into_iter()
        .filter(|path| match path.extension() {
            Some(ext) => ext == "md",
            None => false,
        })
        .map(|path| {
            Ok(File {
                id: 0,
                path: match path.clone().into_os_string().into_string() {
                    Ok(str) => str,
                    Err(e) => {
                        return Err(anyhow::anyhow!(
                            "could not turn path: '{:?}' into utf8-string",
                            e
                        ));
                    }
                },
                last_touched: fs::metadata(&path)?.modified()?.into(),
            })
        })
        .collect()
}

fn fetch_db_files(db: &rusqlite::Connection) -> Result<Vec<File>, anyhow::Error> {
    let mut stmt = db.prepare("SELECT id, path, last_touched FROM files")?;
    stmt.query(rusqlite::params![])?
        .and_then(|row| {
            Ok::<_, anyhow::Error>(File {
                id: row.get(0)?,
                path: row.get(1)?,
                last_touched: row.get(2)?,
            })
        })
        .collect()
}

fn delete_files_from_db(db: &rusqlite::Connection, files: &[i64]) -> Result<(), anyhow::Error> {
    for file in files.iter() {
        db.prepare_cached("DELETE FROM term_frequencies WHERE file = ?;")?
            .execute(rusqlite::params![file])?;
        db.prepare_cached("DELETE FROM todos WHERE file = ?;")?
            .execute(rusqlite::params![file])?;
        db.prepare_cached("DELETE FROM files WHERE id = ?;")?
            .execute(rusqlite::params![file])?;
    }

    Ok(())
}

/// removes files which are present in the db which are no longer present in directory
fn db_garbage_collect(
    db: &rusqlite::Connection,
    dir_files: &[File],
    db_files: &[File],
) -> Result<(), anyhow::Error> {
    let dir_file_set: HashSet<&str> = dir_files.iter().map(|file| file.path.as_str()).collect();

    let db_files: Vec<_> = db_files
        .iter()
        .filter(|file| !dir_file_set.contains(file.path.as_str()))
        .map(|file| file.id)
        .collect();

    delete_files_from_db(db, &db_files)
}

/// deletes every file which has changed from the database and returns a list of their paths
fn discover(
    db: &rusqlite::Connection,
    dir: std::fs::ReadDir,
) -> Result<Vec<PathBuf>, anyhow::Error> {
    let db_files = fetch_db_files(db)?;
    let dir_files = discover_files(dir)?;

    db_garbage_collect(db, &dir_files, &db_files)?;

    let db_files_map: HashMap<_, _> = db_files
        .iter()
        .map(|file| (file.path.as_str(), file))
        .collect();

    let files: Vec<File> = dir_files
        .into_iter()
        .filter(|dir_file| {
            dir_file.last_touched
                > db_files_map
                    .get(dir_file.path.as_str())
                    .map(|file| file.last_touched)
                    .unwrap_or(TimeStamp::new(0))
        })
        .collect();

    let ids: Vec<_> = files
        .iter()
        .map(|file| db_files_map.get(file.path.as_str()))
        .filter(|file| file.is_some())
        .map(|file| file.unwrap().id)
        .collect();

    delete_files_from_db(db, &ids)?;

    let paths: Vec<PathBuf> = files.into_iter().map(|file| file.path.into()).collect();

    Ok(paths)
}

fn get_or_insert_term(db: &rusqlite::Connection, term: &str) -> Result<i64, anyhow::Error> {
    let exists: bool = db
        .prepare_cached("SELECT EXISTS (SELECT 1 FROM terms WHERE term = ?);")?
        .query_row(rusqlite::params![term], |row| row.get(0))?;

    let id: i64 = if !exists {
        db.prepare_cached("INSERT INTO terms (term) VALUES(?) RETURNING id;")?
            .query_row(rusqlite::params![term], |row| row.get(0))?
    } else {
        db.prepare_cached("SELECT id FROM terms WHERE term = ?;")?
            .query_row(rusqlite::params![term], |row| row.get(0))?
    };

    Ok(id)
}

fn index_file(db: &rusqlite::Connection, path: &Path) -> Result<(), anyhow::Error> {
    let file_str = match path.to_owned().into_os_string().into_string() {
        Ok(p) => p,
        Err(e) => return Err(anyhow::anyhow!("could not convert path to string: {:?}", e)),
    };

    let mtime: TimeStamp = fs::metadata(path)?.modified()?.into();

    let file_id: i64 = db
        .prepare_cached("INSERT INTO files (path, last_touched) VALUES(?, ?) RETURNING id;")?
        .query_row(rusqlite::params![&file_str, mtime], |row| row.get(0))?;

    let body = fs::read_to_string(path)?;

    let todos = todo::parse_todos(
        &body,
        path.extension()
            .map(|v| FileFormat::new(v))
            .unwrap_or(FileFormat::Unknown),
    )?;

    for todo in todos.iter() {
        db.prepare_cached(
            "INSERT INTO todos (file, title, deadline, scheduled) VALUES(?, ?, ?, ?);",
        )?
        .execute(rusqlite::params![
            file_id,
            todo.title,
            todo.deadline,
            todo.scheduled
        ])?;
    }

    let mut histogram: HashMap<String, i64> = HashMap::new();

    let mut count = 0;

    for term in body.split_whitespace() {
        let term = stemmer::stem(term);
        *histogram.entry(term).or_insert(0) += 1;
        count += 1;
    }

    for (term, tf) in histogram.iter() {
        let term_id = get_or_insert_term(db, term)?;
        let tf: f64 = *tf as f64 / count as f64;

        db.prepare_cached("INSERT INTO term_frequencies (term, file, tf) VALUES(?, ?, ?);")?
            .execute(rusqlite::params![term_id, file_id, tf])?;
    }

    Ok(())
}

pub fn index(db: &rusqlite::Connection, dir: fs::ReadDir) -> Result<(), anyhow::Error> {
    discover(db, dir)?
        .into_iter()
        .try_for_each(|path| index_file(db, &path))
}
