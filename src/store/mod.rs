use crate::time_stamp::TimeStamp;
use std::path::{Path, PathBuf};

mod id;
pub use id::ID;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct File {
    pub id: ID<Self>,
    pub path: PathBuf,
    pub modified: TimeStamp,
}

impl id::IDAble for File {}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Term {
    pub id: ID<Self>,
    pub term: String,
}

impl id::IDAble for Term {}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TermFrequency {
    pub term: ID<Term>,
    pub file: ID<File>,
    pub frequency: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InverseDocumentFrequency {
    pub term: ID<Term>,
    pub frequency: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Store {
    pub file_id_max: u64,
    pub term_id_max: u64,
    pub files: Vec<File>,
    pub terms: Vec<Term>,
    pub term_frequencies: Vec<TermFrequency>,
    pub inverse_document_frequencies: Vec<InverseDocumentFrequency>,
    pub todos: Vec<Todo>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub file: ID<File>,
    pub line_number: usize,
    pub title: String,
    pub deadline: Option<TimeStamp>,
    pub scheduled: Option<TimeStamp>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            file_id_max: 0,
            term_id_max: 0,
            files: Vec::new(),
            terms: Vec::new(),
            term_frequencies: Vec::new(),
            inverse_document_frequencies: Vec::new(),
            todos: Vec::new(),
        }
    }
}

impl Store {
    pub fn open(path: &Path) -> Result<Store, anyhow::Error> {
        let store = match std::fs::read_to_string(path) {
            Ok(body) => Some(body),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => None,
                std::io::ErrorKind::PermissionDenied => panic!(
                    "You don't have persmissions to read your own store! What happened? Owl store at: {:?}",
                    path
                ),
                _ => panic!("could not open owl store at: {:?}", path),
            },
        };

        if let Some(store) = store {
            Ok(serde_json::from_str(&store)?)
        } else {
            Ok(Store::default())
        }
    }

    pub fn close(&self, path: &Path) -> Result<(), anyhow::Error> {
        let store = serde_json::to_string(self)?;
        std::fs::write(path, &store)?;
        Ok(())
    }

    pub fn file_id(&mut self) -> ID<File> {
        self.file_id_max += 1;
        ID::new(self.file_id_max)
    }

    pub fn term_id(&mut self) -> ID<Term> {
        self.term_id_max += 1;
        ID::new(self.term_id_max)
    }
}
