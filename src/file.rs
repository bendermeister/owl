use crate::time;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct File {
    pub path: PathBuf,
    pub mtime: time::Stamp,
}
