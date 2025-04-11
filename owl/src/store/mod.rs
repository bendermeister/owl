use crate::timestamp::TimeStamp;
use crate::todo::Todo;
use std::fs;
use std::path::{Path, PathBuf};

mod db;

pub struct Store {
    db: rusqlite::Connection,
    path: PathBuf,
}

fn path_from_todo(base: &Path, todo: &Todo) -> PathBuf {
    let mut filename: String = todo
        .title
        .chars()
        .map(|c| {
            if c.is_alphabetic() && c.is_ascii() {
                c.to_ascii_lowercase()
            } else if c.is_ascii_digit() {
                c
            } else {
                '_'
            }
        })
        .collect();

    filename.push('_');
    filename.push_str(&todo.id.to_string());
    filename.push_str(".md");

    let mut base = base.to_owned();

    base.push(&filename);
    base
}

impl Store {
    pub fn open(path: PathBuf) -> Result<Self, anyhow::Error> {
        let mut db_path = path.clone();
        db_path.push("store.sqlite");
        let db = rusqlite::Connection::open(&db_path)?;
        mittelmeer::migrate(&db)?;
        Ok(Self { db, path })
    }

    pub fn store(&mut self, todo: Todo) -> Result<(), anyhow::Error> {
        let path = path_from_todo(&self.path, &todo);
        let body = todo.generate_body();
        fs::write(&path, &body)?;
        db::todo_insert(&self.db, &todo, &path)?;
        Ok(())
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>, anyhow::Error> {
        db::todo_fetch_all(&self.db)
    }

    pub fn update(&mut self) -> Result<(), anyhow::Error> {
        let mut todos = self.get_todos()?;
        let map = db::todo_path_map(&self.db)?;

        for todo in todos.iter_mut() {
            // TODO: are we allowed to unwrap here?
            
            let (path, last_update) = map.get(&todo.id).unwrap();
            let last_modified: TimeStamp = fs::metadata(path)?.modified()?.try_into()?;
            if *last_update >= last_modified {
                continue;
            }

            let body = fs::read_to_string(path)?;
            todo.update_from_body(&body)?;

            db::todo_update(&self.db, todo)?;
        }

        Ok(())
    }
}
