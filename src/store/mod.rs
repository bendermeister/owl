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

fn create_default_store(path: &Path) -> Store {
    let store = Store::default();

    if let Some(parent) = path.parent() {
        log::info!("mkdir --parents {:?}", parent);
        match std::fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(err) => panic!(
                "could not create parent directories of default store at: {:?}: error: {:?}",
                path, err
            ),
        };

        let store_body = match serde_json::to_string(&store) {
            Ok(body) => body,
            Err(err) => panic!("could not serialize store to json: error: {:?}", err),
        };
        log::info!("serialized store to json");

        match std::fs::write(path, store_body) {
            Ok(_) => (),
            Err(err) => panic!("could not write store to {:?}: error: {:?}", path, err),
        };
        log::info!("wrote serialized store to {:?}", path);
    }

    store
}

impl Store {
    pub fn open(path: &Path) -> Store {
        match std::fs::read_to_string(path) {
            Ok(store) => match serde_json::from_str(&store) {
                Ok(store) => store,
                Err(e) => panic!("failed to parse store at: {:?}: error: {:?}", path, e),
            },
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => return create_default_store(path),
                _ => panic!("could not read store at '{:?}': error: {:?}", path, err),
            },
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
