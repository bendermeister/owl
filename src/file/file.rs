use super::prelude::*;
use crate::file_format::FileFormat;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Path {
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct File {
    path: std::path::PathBuf,
}

#[derive(Debug, Clone)]
pub struct Directory {
    path: std::path::PathBuf,
}

impl File {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Path {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

#[allow(refining_impl_trait)]
impl PathLike for Path {
    fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    fn is_file(&self) -> bool {
        self.path.is_file()
    }

    fn to_dir(self) -> Result<Directory, File> {
        if self.path.is_dir() {
            Ok(Directory::new(self.path))
        } else {
            Err(File::new(self.path))
        }
    }

    fn to_file(self) -> Result<File, Directory> {
        if self.path.is_dir() {
            Err(Directory::new(self.path))
        } else {
            Ok(File::new(self.path))
        }
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

#[allow(refining_impl_trait)]
impl DirectoryLike for Directory {
    fn discover(self) -> Vec<File> {
        let dir = self.path.read_dir().unwrap();
        let mut buf = Vec::new();

        for entry in dir {
            let path = entry.unwrap().path();
            if path.is_dir() {
                buf.append(&mut Directory::new(path).discover());
            } else {
                buf.push(File::new(path));
            }
        }

        buf
    }
}

impl FileLike for File {
    fn read(&self) -> String {
        std::fs::read_to_string(&self.path).unwrap()
    }

    fn extension(&self) -> Option<&OsStr> {
        self.path.extension()
    }

    fn modified(&self) -> crate::time_stamp::TimeStamp {
        std::fs::metadata(&self.path)
            .unwrap()
            .modified()
            .unwrap()
            .into()
    }

    fn file_format(&self) -> crate::file_format::FileFormat {
        self.extension()
            .map(FileFormat::new)
            .unwrap_or(FileFormat::Unknown)
    }

    fn path(&self) -> &std::path::Path {
        &self.path
    }
}
