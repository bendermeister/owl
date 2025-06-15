use crate::file::prelude::*;
use crate::store::Store;
use crate::stemmer;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

fn term_histogram_plain_text(file: &impl FileLike) -> HashMap<String, u64> {
    let body = file.read();
    let terms = body.split_whitespace().map(stemmer::stem);

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

pub fn rank(store: &Store, phrase: &str) -> Vec<PathBuf> {
    let term_id_map = store
        .terms
        .iter()
        .map(|t| (t.term.as_str(), t.id))
        .collect::<HashMap<_, _>>();

    let phrase: HashSet<_> = phrase
        .split_whitespace()
        .map(|s| term_id_map.get(stemmer::stem(s).as_str()))
        .filter(|t| t.is_some())
        .map(|t| *t.unwrap())
        .collect();

    let idf = store
        .inverse_document_frequencies
        .iter()
        .map(|idf| (idf.term, idf.frequency))
        .collect::<HashMap<_, _>>();

    let frequencies = store
        .term_frequencies
        .iter()
        .filter(|tf| phrase.contains(&tf.term))
        .map(|tf| (tf.file, tf.frequency * idf.get(&tf.term).unwrap()))
        .collect::<Vec<_>>();

    let mut ranking = HashMap::new();

    for (file, rank) in frequencies.into_iter() {
        *ranking.entry(file).or_insert(0) += rank;
    }

    let file_map = store
        .files
        .iter()
        .map(|file| (file.id, file.path.as_path()))
        .collect::<HashMap<_, _>>();

    let mut ranking = ranking
        .into_iter()
        .map(|(file, rank)| (file_map.get(&file).unwrap().to_path_buf(), rank))
        .collect::<Vec<_>>();

    ranking.sort_by(|a, b| a.1.cmp(&b.1));

    ranking.into_iter().map(|(a, _)| a).collect()

}
