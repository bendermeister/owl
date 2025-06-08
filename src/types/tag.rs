use crate::types::id::Id;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub name: String,
    pub id: Id,
}

impl Tag {
    pub fn new(name: String, id: Id) -> Self {
        Self { name, id }
    }

    pub fn get_tag_id_map(
        db: &rusqlite::Connection,
    ) -> Result<HashMap<String, Tag>, anyhow::Error> {
        todo!()
    }

    pub fn get_id_tag_map(db: &rusqlite::Connection) -> Result<HashMap<Id, Tag>, anyhow::Error> {
        todo!()
    }
}
