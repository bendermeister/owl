use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::{config::Config, file::File, format::Format, store::Store, task, time, todo};

/// recursively discoveres every file starting from `config.base_directory` checks if it needs to
/// be reparsed based on the `mtime` stored in the associated `store.files` and updates
/// `store.tasks` and `store.todos` accordingly
///
/// # Panics
/// - function panics if `config.base_directory` is not a valid path to a directory
/// - function panics if it fails to read a directory in the recursive scan
pub fn index(store: &mut Store, config: &Config) {
    if !config.base_directory.is_dir() {
        panic!(
            "expected base directory: {:?} to be a directory",
            &config.base_directory
        );
    }

    let mut mtime_map = store
        .files
        .iter_mut()
        .map(|file| (file.path.as_path(), &mut file.mtime))
        .collect::<HashMap<_, _>>();

    let mut directories = vec![config.base_directory.clone()];
    let mut files = vec![];

    let now = time::Stamp::now();

    while let Some(directory) = directories.pop() {
        let read_dir = match std::fs::read_dir(&directory) {
            Ok(read_dir) => read_dir,
            Err(err) => panic!(
                "could not read entries of directory: {:?} because: {:?}",
                directory, err
            ),
        };

        'entry_loop: for entry in read_dir {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    log::warn!("ignoring entry in {:?} because {:?}", directory, err);
                    continue;
                }
            };

            let path = entry.path();

            if config.ignore_hidden_files {
                let name = path
                    .components()
                    .next_back()
                    .unwrap()
                    .as_os_str()
                    .as_encoded_bytes();
                if name[0] == b'.' {
                    log::info!("ignoring hidden path: {:?}", path);
                    continue;
                }
            }

            for glob in config.ignore.iter() {
                if fast_glob::glob_match(glob, path.as_path().as_os_str().as_encoded_bytes()) {
                    log::info!("ignoring path: {:?}", path);
                    continue 'entry_loop;
                }
            }

            if path.is_dir() {
                directories.push(path);
                continue 'entry_loop;
            }

            if !path.is_file() {
                log::warn!(
                    "ignoring path {:?} because it is neither directory nor file",
                    path
                );
                continue 'entry_loop;
            }

            if Format::new(&path).is_unknown() {
                log::info!("ignoring file: {:?} because format is unknown", path);
                continue 'entry_loop;
            }

            let mtime = match std::fs::metadata(&path) {
                Ok(mtime) => mtime,
                Err(err) => {
                    log::warn!(
                        "ignoring file: {:?} because: failed to read metadata: {:?}",
                        path,
                        err
                    );
                    continue;
                }
            };

            // unwrap is ok because it only fails if mtime is not available on platform
            // we don't support such platforms
            let mtime: time::Stamp = mtime.modified().unwrap().into();

            if let Some(last_mtime) = mtime_map.get_mut(path.as_path()) {
                if **last_mtime >= mtime {
                    log::info!(
                        "ignoring: file {:?} because it has not changed since last scan",
                        path
                    );
                    **last_mtime = now;
                    continue;
                }
            }

            files.push((path, mtime));
        }
    }

    // remove files from store which will be updated

    let file_set = files
        .iter()
        .map(|(path, _)| path.as_path())
        .collect::<HashSet<_>>();

    let to_remove = store
        .files
        .iter()
        .filter(|file| file.mtime < now)
        .map(|file| file.path.as_path())
        .collect::<HashSet<_>>();

    let retain = |path: &Path| !file_set.contains(path) && !to_remove.contains(path);

    store.tasks.retain(|task| retain(task.path.as_path()));
    store.todos.retain(|todo| retain(todo.path.as_path()));

    store
        .files
        .retain(|file| !file_set.contains(file.path.as_path()) && file.mtime >= now);

    for (path, mtime) in files.into_iter() {
        let body = match std::fs::read_to_string(&path) {
            Ok(body) => body,
            Err(err) => {
                log::warn!("ignoring file: {:?} because: {:?}", &path, err);
                continue;
            }
        };

        let todos = todo::parse(&body, &path).into_iter();
        let tasks = task::Task::parse(&body, &path).into_iter();

        store.todos.extend(todos);
        store.tasks.extend(tasks);
        store.files.push(File { path, mtime });
    }
}
