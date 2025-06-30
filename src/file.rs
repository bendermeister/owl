use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct File {
    pub path: PathBuf,
    pub mtime: SystemTime,
}
