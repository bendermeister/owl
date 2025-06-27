use crate::file::File;
use crate::task::Task;
use crate::todo::Todo;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub struct Store {
    pub files: Vec<File>,
    pub todos: Vec<Todo>,
    pub tasks: Vec<Task>,
}

impl Store {
    /// reads and parses a json encoded store at `path`
    ///
    /// # Panics
    /// if reading, or parsing fails the function panics with a meaningful error message
    pub fn open(path: &Path) -> Self {
        let store = match std::fs::read_to_string(path) {
            Ok(store) => store,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => return Self::default(),
                _ => panic!("could not read store at: {:?}: {:?}", path, err),
            },
        };
        log::info!("read store at: {:?}", path);

        let store: Store = match serde_json::from_str(&store) {
            Ok(store) => store,
            Err(err) => panic!("could not read store at: {:?}: {:?}", path, err),
        };

        log::info!("deserialized store from string");

        store
    }

    /// writes the store as json to `path`
    ///
    /// # Panics
    /// if writing or serializing causes an error the function panics with a meaningful error
    /// message
    pub fn close(&self, path: &Path) {
        let store = match serde_json::to_string(self) {
            Ok(store) => store,
            Err(err) => panic!("could not save store to {:?} because: {:?}", path, err),
        };

        log::info!("serialized store to json");

        // TODO: do we have to create parent directories?

        match std::fs::write(path, store) {
            Ok(_) => (),
            Err(err) => panic!("could not save store to {:?} because {:?}", path, err),
        };

        log::info!("wrote store to {:?}", path);
    }
}
