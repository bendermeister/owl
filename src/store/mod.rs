use crate::time;
use std::path::{Path, PathBuf};

mod id;
pub use id::ID;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct File {
    pub id: ID<Self>,
    pub path: PathBuf,
    pub modified: time::Stamp,
}

impl id::IDAble for File {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Store {
    pub file_id_max: u64,
    pub term_id_max: u64,
    pub files: Vec<File>,
    pub todos: Vec<Todo>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub file: ID<File>,
    pub line_number: usize,
    pub title: String,
    pub deadline: Option<time::Stamp>,
    pub scheduled: Option<time::Stamp>,
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
                std::io::ErrorKind::NotFound => create_default_store(path),
                _ => panic!("could not read store at '{:?}': error: {:?}", path, err),
            },
        }
    }

    pub fn close(&self, path: &Path) -> Result<(), std::io::Error> {
        let store = serde_json::to_string(self)?;
        std::fs::write(path, &store)?;
        Ok(())
    }

    pub fn file_id(&mut self) -> ID<File> {
        self.file_id_max += 1;
        ID::new(self.file_id_max)
    }
}
