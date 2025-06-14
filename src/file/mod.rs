use crate::file_format::FileFormat;
use crate::time_stamp::TimeStamp;
use std::ffi::OsStr;
use std::path::PathBuf;

pub trait PathLike {
    fn path(&self) -> PathBuf;
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn to_dir(self) -> Result<impl DirectoryLike, impl FileLike>;
    fn to_file(self) -> Result<impl FileLike, impl DirectoryLike>;
}

pub trait FileLike {
    fn read(&self) -> String;
    fn extension(&self) -> Option<&OsStr>;
    fn file_format(&self) -> FileFormat;
    fn modified(&self) -> TimeStamp;
    fn path(&self) -> &std::path::Path;
}

pub trait DirectoryLike {
    fn discover(self) -> Vec<impl FileLike>;
}

pub mod prelude {
    pub use super::{DirectoryLike, FileLike, PathLike};
}

pub mod file;
pub mod test_file;
