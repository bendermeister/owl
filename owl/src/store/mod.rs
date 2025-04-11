use crate::id::ID;
use crate::tag::Tag;
use crate::timestamp::TimeStamp;
use crate::todo::Todo;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

mod db;

pub struct Store {
    pub db: rusqlite::Connection,
    pub path: PathBuf,
    pub tags: HashMap<String, ID<Tag>>,
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

fn assign_tags(mut tags: HashMap<ID<Todo>, HashSet<ID<Tag>>>, todos: &mut [Todo]) {
    for todo in todos.iter_mut() {
        todo.tags = match tags.remove(&todo.id) {
            Some(tags) => tags,
            None => continue,
        }
    }
}

impl Store {
    pub fn open(path: PathBuf) -> Result<Self, anyhow::Error> {
        let mut db_path = path.clone();
        db_path.push("store.sqlite");
        let db = rusqlite::Connection::open(&db_path)?;
        mittelmeer::migrate(&db)?;

        let tags = db::tags_fetch_all(&db)?;

        Ok(Self { db, path, tags })
    }

    pub fn store_todo(&mut self, todo: Todo) -> Result<(), anyhow::Error> {
        let path = path_from_todo(&self.path, &todo);
        let tags = self.tags.iter().map(|(k, v)| (*v, k.as_str())).collect();
        let body = todo.generate_body(&tags);
        fs::write(&path, &body)?;
        db::todo_insert(&self.db, &todo, &path)?;
        Ok(())
    }

    pub fn store_tag(&mut self, tag: Tag) -> Result<(), anyhow::Error> {
        db::tag_insert(&self.db, &tag)
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>, anyhow::Error> {
        let mut todos = db::todo_fetch_all(&self.db)?;
        let tag_mapping = db::tag_mapping_fetch_all(&self.db)?;
        assign_tags(tag_mapping, &mut todos);

        Ok(todos)
    }

    pub fn update(&mut self, force: bool) -> Result<(), anyhow::Error> {
        let mut todos = self.get_todos()?;
        let map = db::todo_path_map(&self.db)?;

        for todo in todos.iter_mut() {
            // TODO: are we allowed to unwrap here?

            let (path, last_update) = map.get(&todo.id).unwrap();
            let last_modified: TimeStamp = fs::metadata(path)?.modified()?.try_into()?;
            // TODO: this force thing is a bit unclean
            if !force && *last_update >= last_modified {
                continue;
            }

            let body = fs::read_to_string(path)?;
            todo.update_from_body(&self.tags, &body)?;

            db::tag_mapping_clear(&self.db, todo.id)?;
            for tag in todo.tags.iter() {
                db::tag_mapping_insert(&self.db, todo.id, *tag)?;
            }

            db::todo_update(&self.db, todo)?;
        }

        Ok(())
    }
}
