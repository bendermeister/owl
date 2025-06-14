use crate::file::prelude::*;
use crate::tfidf;
use crate::time_stamp::TimeStamp;
use crate::todo;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

struct DBFile {
    id: i64,
    path: PathBuf,
    modified: TimeStamp,
}

fn db_delete_files(db: &rusqlite::Connection, files: &[i64]) -> Result<(), anyhow::Error> {
    for id in files.iter() {
        db.prepare_cached("DELETE FROM term_frequencies WHERE file = ?")?
            .execute([id])?;

        db.prepare_cached("DELETE FROM todos WHERE file = ?")?
            .execute([id])?;

        db.prepare_cached("DELETE FROM files WHERE id = ?")?
            .execute([id])?;
    }

    Ok(())
}

fn index_file(db: &rusqlite::Connection, file: impl FileLike) -> Result<(), anyhow::Error> {
    let file_id: i64 = db
        .prepare_cached("INSERT INTO files (path, modified) VALUES(?, ?) RETURNING id;")?
        .query_row(
            rusqlite::params![file.path().to_string_lossy(), file.modified()],
            |row| row.get(0),
        )?;

    // find all todos in file body and insert them into database

    let todos = todo::parse_todos(&file)?;

    for todo in todos.into_iter() {
        db.prepare_cached(
            "
            INSERT INTO todos 
                (file, title, deadline, scheduled, line)
                VALUES(?, ?, ?, ?, ?);
            ",
        )?
        .execute(rusqlite::params![
            file_id,
            todo.title,
            todo.deadline,
            todo.scheduled,
            todo.line_number,
        ])?;
    }

    // find all terms in file body and insert them into database

    let terms = tfidf::term_histogram(&file);

    for (term, frequency) in terms.into_iter() {
        let exists: bool = db
            .prepare_cached("SELECT EXISTS (SELECT 1 FROM terms WHERE term = ? LIMIT 1);")?
            .query_row([&term], |row| row.get(0))?;

        let term_id: i64 = if exists {
            db.prepare_cached("SELECT id FROM terms WHERE term = ? LIMIT 1;")?
                .query_row([&term], |row| row.get(0))?
        } else {
            db.prepare_cached("INSERT INTO terms (term) VALUES(?) RETURNING id;")?
                .query_row([&term], |row| row.get(0))?
        };

        db.prepare_cached("INSERT INTO term_frequencies (term, file, tf) VALUES(?, ?, ?);")?
            .execute(rusqlite::params![term_id, file_id, frequency])?;
    }

    Ok(())
}

pub fn index(db: &rusqlite::Connection, dir: impl DirectoryLike) -> Result<(), anyhow::Error> {
    let files = dir.discover();

    let file_set = files.iter().map(|f| f.path()).collect::<HashSet<_>>();

    // get every file from database
    //

    let db_files = db
        .prepare("SELECT id, path, modified FROM files;")?
        .query([])?
        .and_then(|row| {
            Ok(DBFile {
                id: row.get(0)?,
                path: row.get::<_, String>(1)?.into(),
                modified: row.get(2)?,
            })
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    // delete every file from database which is no longer present in actual file tree

    let db_garbage = db_files
        .iter()
        .filter(|f| !file_set.contains(f.path.as_path()))
        .map(|f| f.id)
        .collect::<Vec<_>>();

    db_delete_files(db, &db_garbage)?;

    // find every file which was updated or created since last index

    let db_file_map: HashMap<&std::path::Path, &DBFile> =
        db_files.iter().map(|f| (f.path.as_path(), f)).collect();

    let files = files
        .into_iter()
        .filter(|f| {
            f.modified()
                > db_file_map
                    .get(f.path())
                    .map(|v| v.modified)
                    .unwrap_or(TimeStamp::new(0))
        })
        .collect::<Vec<_>>();

    // those files can be deleted from the database as they will be reiniserted afterwards

    let db_garbage = files
        .iter()
        .map(|f| db_file_map.get(f.path()))
        .filter(|f| f.is_some())
        .map(|f| f.unwrap().id)
        .collect::<Vec<_>>();

    db_delete_files(db, &db_garbage)?;

    // index those files

    for file in files.into_iter() {
        index_file(db, file)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::file::test_file::{TestBody, TestDirectory, TestFile, TestPath};
    use crate::todo::Todo;

    #[test]
    fn test_index() {
        let db = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::migration::migrate(&db).unwrap();

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

        index(&db, root).unwrap();

        let mut todos = db
            .prepare("
                SELECT 
                    todos.title, 
                    todos.deadline, 
                    todos.scheduled, 
                    todos.line,
                    files.path
                FROM todos INNER JOIN files ON todos.file = files.id;")
            .unwrap()
            .query([])
            .unwrap()
            .and_then(|row| {
                Ok(Todo {
                    title: row.get(0).unwrap(),
                    deadline: row.get(1).unwrap(),
                    scheduled: row.get(2).unwrap(),
                    line_number: row.get(3).unwrap(),
                    file: row.get::<_, String>(4).unwrap().into(),
                })
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()
            .unwrap();

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

        index(&db, root).unwrap();

        let mut todos = db
            .prepare(
                "
                SELECT 
                    todos.title, 
                    todos.deadline, 
                    todos.scheduled, 
                    todos.line,
                    files.path
                FROM todos INNER JOIN files ON todos.file = files.id;",
            )
            .unwrap()
            .query([])
            .unwrap()
            .and_then(|row| {
                Ok(Todo {
                    title: row.get(0).unwrap(),
                    deadline: row.get(1).unwrap(),
                    scheduled: row.get(2).unwrap(),
                    line_number: row.get(3).unwrap(),
                    file: row.get::<_, String>(4).unwrap().into(),
                })
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()
            .unwrap();

        todos.sort_by(|a, b| a.title.cmp(&b.title));

        let mut expected = Vec::new();
        expected.append(&mut todo::parse_todos(&file1).unwrap());
        expected.append(&mut todo::parse_todos(&file2).unwrap());
        expected.append(&mut todo::parse_todos(&file3).unwrap());
        expected.sort_by(|a, b| a.title.cmp(&b.title));

        assert_eq!(expected, todos);
    }
}
