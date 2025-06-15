use crate::file::prelude::*;
use crate::file_format::FileFormat;
use crate::store;
use crate::store::Store;
use crate::tfidf;
use crate::time_stamp::TimeStamp;
use crate::todo;
use std::collections::{HashMap, HashSet};

pub fn index(store: &mut Store, dir: impl DirectoryLike) -> Result<(), anyhow::Error> {
    // TODO: dir.discover should return iterator
    let files = dir.discover();

    // filter out any files with unknown format
    let mut files = files
        .into_iter()
        .filter(|f| match f.file_format() {
            FileFormat::Unknown => false,
            FileFormat::Markdown => true,
            FileFormat::Typst => true,
        })
        .collect::<Vec<_>>();

    let file_set = files.iter().map(|f| f.path()).collect::<HashSet<_>>();

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
        f.modified()
            > store_map
                .get(f.path())
                .map(|v| v.modified)
                .unwrap_or(TimeStamp::new(0))
    });

    // those files need to be removed from the store
    let file_set = files.iter().map(|file| file.path()).collect::<HashSet<_>>();
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

    for file in files.into_iter() {
        let todos = todo::parse_todos(&file)?;
        let term_frequencies = tfidf::term_histogram(&file);
        let file_id = store.file_id();

        store.files.push(store::File {
            id: file_id,
            path: file.path().to_path_buf(),
            modified: file.modified(),
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

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::file::test_file::{TestBody, TestDirectory, TestFile, TestPath};

    #[test]
    fn test_index() {
        let mut store = Store {
            file_id_max: 0,
            term_id_max: 0,
            files: Vec::new(),
            terms: Vec::new(),
            term_frequencies: Vec::new(),
            inverse_document_frequencies: Vec::new(),
            todos: Vec::new(),
        };

        let file1 = TestFile {
            path: "/root/dir1/todo1.md".into(),
            body: "# TODO: todo1".into(),
            modified: TimeStamp::new(200),
        };

        let file2 = TestFile {
            path: "/root/dir1/todo2.md".into(),
            body: "# TODO: todo2".into(),
            modified: TimeStamp::new(200),
        };

        let dir1 = TestDirectory {
            path: "/root/dir1/".into(),
            body: vec![
                TestPath {
                    body: TestBody::File(file1.clone()),
                },
                TestPath {
                    body: TestBody::File(file2.clone()),
                },
            ],
        };

        let file3 = TestFile {
            path: "/root/todo3.typ".into(),
            body: "= TODO: todo3".into(),
            modified: TimeStamp::new(200),
        };

        let root = TestDirectory {
            path: "/root/".into(),
            body: vec![
                TestPath {
                    body: TestBody::File(file3.clone()),
                },
                TestPath {
                    body: TestBody::Directory(dir1.clone()),
                },
            ],
        };

        index(&mut store, root).unwrap();

        let file_map = store
            .files
            .iter()
            .map(|file| (file.id, file.path.as_path()))
            .collect::<HashMap<_, _>>();

        let mut todos = store
            .todos
            .iter()
            .map(|t| crate::todo::Todo {
                title: t.title.to_owned(),
                file: file_map.get(&t.file).unwrap().to_path_buf(),
                deadline: t.deadline.clone(),
                scheduled: t.scheduled.clone(),
                line_number: t.line_number,
            })
            .collect::<Vec<_>>();

        todos.sort_by(|a, b| a.title.cmp(&b.title));

        let mut expected = Vec::new();
        expected.append(&mut todo::parse_todos(&file1).unwrap());
        expected.append(&mut todo::parse_todos(&file2).unwrap());
        expected.append(&mut todo::parse_todos(&file3).unwrap());
        expected.sort_by(|a, b| a.title.cmp(&b.title));

        assert_eq!(expected, todos);

        // todo if update goes through

        let file1 = TestFile {
            path: "/root/dir1/todo1.md".into(),
            body: "# TODO: update todo1".into(),
            modified: TimeStamp::new(300),
        };

        let file2 = TestFile {
            path: "/root/dir1/todo2.md".into(),
            body: "# TODO: update todo2".into(),
            modified: TimeStamp::new(300),
        };

        let dir1 = TestDirectory {
            path: "/root/dir1/".into(),
            body: vec![
                TestPath {
                    body: TestBody::File(file1.clone()),
                },
                TestPath {
                    body: TestBody::File(file2.clone()),
                },
            ],
        };

        let root = TestDirectory {
            path: "/root/".into(),
            body: vec![
                TestPath {
                    body: TestBody::File(file3.clone()),
                },
                TestPath {
                    body: TestBody::Directory(dir1.clone()),
                },
            ],
        };

        index(&mut store, root).unwrap();


        let file_map = store
            .files
            .iter()
            .map(|file| (file.id, file.path.as_path()))
            .collect::<HashMap<_, _>>();

        let mut todos = store
            .todos
            .iter()
            .map(|t| crate::todo::Todo {
                title: t.title.to_owned(),
                file: file_map.get(&t.file).unwrap().to_path_buf(),
                deadline: t.deadline.clone(),
                scheduled: t.scheduled.clone(),
                line_number: t.line_number,
            })
            .collect::<Vec<_>>();

        todos.sort_by(|a, b| a.title.cmp(&b.title));

        let mut expected = Vec::new();
        expected.append(&mut todo::parse_todos(&file1).unwrap());
        expected.append(&mut todo::parse_todos(&file2).unwrap());
        expected.append(&mut todo::parse_todos(&file3).unwrap());
        expected.sort_by(|a, b| a.title.cmp(&b.title));

        assert_eq!(expected, todos);
    }
}
