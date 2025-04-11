use crate::id::ID;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tag {
    pub id: ID<Tag>,
    pub name: String,
}

impl Tag {
    pub fn new(id: ID<Tag>, name: String) -> Self {
        Self { id, name }
    }
}
