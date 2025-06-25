use crate::config::Config;
use crate::file_format::FileFormat;
use crate::store;
use crate::store::Store;
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

            if config.ignore_hidden_files {
                // TODO: this cannot be the right way
                if entry.file_name().as_os_str().as_encoded_bytes()[0] == b'.' {
                    continue;
                }
            }

            let file_path = entry.path();
            for glob in config.ignore.iter() {
                if glob_match(
                    glob.as_os_str().as_encoded_bytes(),
                    file_path.as_os_str().as_encoded_bytes(),
                ) {
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

    // Set of files currently present in the directory
    let file_set = files
        .iter()
        .map(|file| file.path.as_path())
        .collect::<HashSet<_>>();

    // Remove all files from store which are no longer present on the file system
    store
        .files
        .retain(|file| file_set.contains(file.path.as_path()));

    // maps files to their modification time present in the store
    let store_modified = store
        .files
        .iter()
        .map(|file| (file.path.as_path(), file.modified))
        .collect::<HashMap<_, _>>();

    // keep only files which need to be updated
    files.retain(|file| {
        file.modified
            > store_modified
                .get(file.path.as_path())
                .map(|v| *v)
                .unwrap_or_else(|| time::Stamp::new(0))
    });

    // remove every file which needs to be updated from the store
    let file_set = files
        .iter()
        .map(|file| file.path.as_path())
        .collect::<HashSet<_>>();
    store
        .files
        .retain(|file| !file_set.contains(file.path.as_path()));

    let files = files.into_iter().map(|file| (file.path, file.modified));
    for (path, modified) in files {
        let body = match std::fs::read_to_string(&path) {
            Ok(body) => body,
            Err(err) => {
                log::warn!("failed to read file: {:?}: {:?} ignoring", path, err);
                continue;
            }
        };
        let todos = match todo::parse_todos(&body, FileFormat::new(&path)) {
            Ok(todos) => todos,
            Err(err) => {
                log::warn!(
                    "failed to parse todos in file: {:?}: {:?} ignoring file",
                    path,
                    err
                );
                continue;
            }
        };

        let file = store::File {
            path,
            modified,
            todos,
        };

        store.files.push(file);

        log::info!("finished indexing: {:?}", &path);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_glob_matching() {
        let glob: PathBuf = "**/node_modules/**".into();
        let path: PathBuf = "/home/user/Repo/project/node_modules/file.js".into();
        assert!(fast_glob::glob_match(
            glob.as_os_str().as_encoded_bytes(),
            path.as_os_str().as_encoded_bytes()
        ));
    }

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
