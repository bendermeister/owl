use crate::todo::Todo;
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
            } else if c.is_digit(10) {
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
        let db = rusqlite::Connection::open(&path)?;
        Ok(Self { db, path })
    }

    pub fn store(&mut self, todo: Todo) -> Result<(), anyhow::Error> {
        let path = path_from_todo(&self.path, &todo);
        db::todo_insert(&self.db, &todo, &path)?;
        Ok(())
    }

    pub fn update(&mut self, todo: Todo) -> Result<(), anyhow::Error> {
    }
}
