use crate::config::Config;
use crate::file_format::FileFormat;
use crate::store;
use crate::store::Store;
use crate::tfidf;
use crate::time;
use crate::todo;
use fast_glob::glob_match;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

struct DirFile {
    path: PathBuf,
    modified: time::Stamp,
}

pub fn index(config: &Config, store: &mut Store, path: &Path) {
    // find all suitable files in path
    if !path.is_dir() {
        panic!(
            "cannot index: provided path is not a direcotry: '{:?}'",
            path
        );
    }

    let mut files = Vec::new();
    let mut dirs = Vec::new();
    dirs.push(path.to_path_buf());

    while let Some(dir_path) = dirs.pop() {
        let dir = match std::fs::read_dir(&dir_path) {
            Ok(dir) => dir,
            Err(err) => {
                log::warn!(
                    "ignoring read_dir error for: {:?}: error: {:?}",
                    dir_path,
                    err
                );
                continue;
            }
        };
        'entry_loop: for entry in dir {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    log::warn!(
                        "ignoring read_dir entry error for: {:?}: error: {:?}",
                        dir_path,
                        err
                    );
                    continue;
                }
            };

            // TODO: this cannot be the right way
            if entry.file_name().as_os_str().as_encoded_bytes()[0] == b'.' {
                continue;
            }

            let file_path = entry.path();

            for ignore_path in config.ignore.iter() {
                // if glob::matches(file_path.as_os_str().as_encoded_bytes(), ignore_path) {
                //     continue 'entry_loop;
                // }
                if glob_match(file_path.as_os_str().as_encoded_bytes(), ignore_path) {
                    continue 'entry_loop;
                }
            }

            if file_path.is_dir() {
                log::info!("discovered directory: {:?}", file_path);
                dirs.push(file_path);
                continue;
            }

            if FileFormat::new(&file_path).is_known() {
                log::info!("discovered file: {:?}", file_path);
                let meta = match std::fs::metadata(&file_path) {
                    Ok(meta) => meta,
                    Err(err) => {
                        log::warn!(
                            "ignoring error while reading metadata for: '{:?}': error: {:?}",
                            file_path,
                            err
                        );
                        continue;
                    }
                };

                let modified = match meta.modified() {
                    Ok(modified) => modified,
                    Err(err) => {
                        log::warn!(
                            "ignoring error while reading mtime for '{:?}' error: {:?}",
                            file_path,
                            err
                        );
                        continue;
                    }
                };

                log::info!("read mtime for file: {:?}", file_path);

                files.push(DirFile {
                    path: file_path,
                    modified: modified.into(),
                });
            }
        }
    }

    let file_set = files
        .iter()
        .map(|f| f.path.as_path())
        .collect::<HashSet<_>>();

    // delete every file from database which is no longer present in actual file tree
    store
        .files
        .retain(|file| file_set.contains(file.path.as_path()));

    // find every file which was updated or created since last index

    let store_map = store
        .files
        .iter()
        .map(|file| (file.path.as_path(), file))
        .collect::<HashMap<_, _>>();

    files.retain(|f| {
        f.modified
            > store_map
                .get(f.path.as_path())
                .map(|v| v.modified)
                .unwrap_or(time::Stamp::new(0))
    });

    // those files need to be removed from the store
    let file_set = files
        .iter()
        .map(|file| file.path.as_path())
        .collect::<HashSet<_>>();
    store
        .files
        .retain(|file| !file_set.contains(file.path.as_path()));

    // get a hashset representing the files left in the store
    let store_set = store
        .files
        .iter()
        .map(|file| file.id.clone())
        .collect::<HashSet<_>>();

    // idfs of terms which where present in files which are no longer in the store need to be
    // updated later
    let mut idf_to_update = store
        .term_frequencies
        .iter()
        .filter(|tf| !store_set.contains(&tf.file))
        .map(|tf| (tf.term.clone(), 0))
        .collect::<HashMap<_, _>>();

    store
        .inverse_document_frequencies
        .retain(|idf| !idf_to_update.contains_key(&idf.term));

    // remove term frequencies of files which are no longer in the store from the store
    store
        .term_frequencies
        .retain(|tf| store_set.contains(&tf.file));

    // remove todos of files which are no longer in thes store from the store
    store.todos.retain(|t| store_set.contains(&t.file));

    let files = files
        .into_iter()
        .map(|file| {
            let body = match std::fs::read_to_string(&file.path) {
                Ok(body) => Some(body),
                Err(err) => {
                    log::warn!(
                        "error while trying to read file -> ignoring file: file: {:?}, error: {:?}",
                        file.path,
                        err
                    );
                    None
                }
            };
            (file.path, file.modified, body)
        })
        .filter(|(_, _, b)| b.is_some())
        .map(|(p, m, b)| (p, m, b.unwrap()));

    for (path, modified, body) in files {
        let todos = match todo::parse_todos(&body, &path) {
            Ok(todos) => todos,
            Err(err) => panic!("could not parse todos of: {:?}: error: {:?}", path, err),
        };
        log::info!("parsed todos of: {:?}", path);

        let term_frequencies = tfidf::term_histogram(&body, &path);
        log::info!("parsed term frequencies of: {:?}", path);

        let file_id = store.file_id();

        store.files.push(store::File {
            id: file_id,
            path,
            modified,
        });

        let todos = todos.into_iter().map(move |todo| store::Todo {
            file: file_id,
            line_number: todo.line_number,
            title: todo.title,
            deadline: todo.deadline,
            scheduled: todo.scheduled,
        });
        store.todos.extend(todos);

        let current_terms = store
            .terms
            .iter()
            .map(|t| (t.term.as_str(), t.id.clone()))
            .collect::<HashMap<_, _>>();

        let mut new_terms = Vec::new();
        let mut old_terms = Vec::new();

        for (term, frequency) in term_frequencies.into_iter() {
            if let Some(id) = current_terms.get(term.as_str()) {
                old_terms.push((*id, frequency));
            } else {
                new_terms.push((term, frequency));
            }
        }

        let old_terms = old_terms
            .into_iter()
            .map(|(id, freq)| store::TermFrequency {
                term: id,
                file: file_id,
                frequency: freq,
            });

        store.term_frequencies.extend(old_terms);

        for (term, frequency) in new_terms.into_iter() {
            let term_id = store.term_id();
            idf_to_update.insert(term_id, 0);
            store.terms.push(store::Term { id: term_id, term });
            store.term_frequencies.push(store::TermFrequency {
                term: term_id,
                file: file_id,
                frequency,
            });
        }
    }

    for tf in store.term_frequencies.iter() {
        if let Some(count) = idf_to_update.get_mut(&tf.term) {
            *count += 1;
        }
    }

    let document_count = store.files.len() as u64;

    let idf_to_update =
        idf_to_update
            .into_iter()
            .map(|(term, idf)| store::InverseDocumentFrequency {
                term,
                frequency: document_count - idf,
            });

    store.inverse_document_frequencies.extend(idf_to_update);
}

#[cfg(test)]
mod test {
    // use super::*;

    //
    //     #[test]
    //     fn test_index() {
    //         let mut store = Store {
    //             file_id_max: 0,
    //             term_id_max: 0,
    //             files: Vec::new(),
    //             terms: Vec::new(),
    //             term_frequencies: Vec::new(),
    //             inverse_document_frequencies: Vec::new(),
    //             todos: Vec::new(),
    //         };
    //
    //         let file1 = TestFile {
    //             path: "/root/dir1/todo1.md".into(),
    //             body: "# TODO: todo1".into(),
    //             modified: TimeStamp::new(200),
    //         };
    //
    //         let file2 = TestFile {
    //             path: "/root/dir1/todo2.md".into(),
    //             body: "# TODO: todo2".into(),
    //             modified: TimeStamp::new(200),
    //         };
    //
    //         let dir1 = TestDirectory {
    //             path: "/root/dir1/".into(),
    //             body: vec![
    //                 TestPath {
    //                     body: TestBody::File(file1.clone()),
    //                 },
    //                 TestPath {
    //                     body: TestBody::File(file2.clone()),
    //                 },
    //             ],
    //         };
    //
    //         let file3 = TestFile {
    //             path: "/root/todo3.typ".into(),
    //             body: "= TODO: todo3".into(),
    //             modified: TimeStamp::new(200),
    //         };
    //
    //         let root = TestDirectory {
    //             path: "/root/".into(),
    //             body: vec![
    //                 TestPath {
    //                     body: TestBody::File(file3.clone()),
    //                 },
    //                 TestPath {
    //                     body: TestBody::Directory(dir1.clone()),
    //                 },
    //             ],
    //         };
    //
    //         index(&mut store, root).unwrap();
    //
    //         let file_map = store
    //             .files
    //             .iter()
    //             .map(|file| (file.id, file.path.as_path()))
    //             .collect::<HashMap<_, _>>();
    //
    //         let mut todos = store
    //             .todos
    //             .iter()
    //             .map(|t| crate::todo::Todo {
    //                 title: t.title.to_owned(),
    //                 file: file_map.get(&t.file).unwrap().to_path_buf(),
    //                 deadline: t.deadline.clone(),
    //                 scheduled: t.scheduled.clone(),
    //                 line_number: t.line_number,
    //             })
    //             .collect::<Vec<_>>();
    //
    //         todos.sort_by(|a, b| a.title.cmp(&b.title));
    //
    //         let mut expected = Vec::new();
    //         expected.append(&mut todo::parse_todos(&file1).unwrap());
    //         expected.append(&mut todo::parse_todos(&file2).unwrap());
    //         expected.append(&mut todo::parse_todos(&file3).unwrap());
    //         expected.sort_by(|a, b| a.title.cmp(&b.title));
    //
    //         assert_eq!(expected, todos);
    //
    //         // todo if update goes through
    //
    //         let file1 = TestFile {
    //             path: "/root/dir1/todo1.md".into(),
    //             body: "# TODO: update todo1".into(),
    //             modified: TimeStamp::new(300),
    //         };
    //
    //         let file2 = TestFile {
    //             path: "/root/dir1/todo2.md".into(),
    //             body: "# TODO: update todo2".into(),
    //             modified: TimeStamp::new(300),
    //         };
    //
    //         let dir1 = TestDirectory {
    //             path: "/root/dir1/".into(),
    //             body: vec![
    //                 TestPath {
    //                     body: TestBody::File(file1.clone()),
    //                 },
    //                 TestPath {
    //                     body: TestBody::File(file2.clone()),
    //                 },
    //             ],
    //         };
    //
    //         let root = TestDirectory {
    //             path: "/root/".into(),
    //             body: vec![
    //                 TestPath {
    //                     body: TestBody::File(file3.clone()),
    //                 },
    //                 TestPath {
    //                     body: TestBody::Directory(dir1.clone()),
    //                 },
    //             ],
    //         };
    //
    //         index(&mut store, root).unwrap();
    //
    //
    //         let file_map = store
    //             .files
    //             .iter()
    //             .map(|file| (file.id, file.path.as_path()))
    //             .collect::<HashMap<_, _>>();
    //
    //         let mut todos = store
    //             .todos
    //             .iter()
    //             .map(|t| crate::todo::Todo {
    //                 title: t.title.to_owned(),
    //                 file: file_map.get(&t.file).unwrap().to_path_buf(),
    //                 deadline: t.deadline.clone(),
    //                 scheduled: t.scheduled.clone(),
    //                 line_number: t.line_number,
    //             })
    //             .collect::<Vec<_>>();
    //
    //         todos.sort_by(|a, b| a.title.cmp(&b.title));
    //
    //         let mut expected = Vec::new();
    //         expected.append(&mut todo::parse_todos(&file1).unwrap());
    //         expected.append(&mut todo::parse_todos(&file2).unwrap());
    //         expected.append(&mut todo::parse_todos(&file3).unwrap());
    //         expected.sort_by(|a, b| a.title.cmp(&b.title));
    //
    //         assert_eq!(expected, todos);
    //     }
}
