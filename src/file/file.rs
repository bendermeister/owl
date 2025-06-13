use super::prelude::*;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Path {
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct File {
    path: std::path::PathBuf,
    body: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Directory {
    path: std::path::PathBuf,
}

impl File {
    pub fn new(path: PathBuf) -> Self {
        Self { path, body: None }
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
        // TODO: there needs to be a better way
        let path: PathBuf = self.path.clone().into();
        path.is_dir()
    }

    fn is_file(&self) -> bool {
        // TODO: there needs to be a better way
        let path: PathBuf = self.path.clone().into();
        path.is_file()
    }

    fn to_dir(self) -> Result<Directory, File> {
        let path: PathBuf = self.path.into();

        if path.is_dir() {
            Ok(Directory::new(path))
        } else {
            Err(File::new(path))
        }
    }

    fn to_file(self) -> Result<File, Directory> {
        let path: PathBuf = self.path.into();

        if path.is_dir() {
            Err(Directory::new(path))
        } else {
            Ok(File::new(path))
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
    fn read(&mut self) -> String {
        if let Some(body) = &self.body {
            return body.clone();
        }
        self.body = Some(std::fs::read_to_string(&self.path).unwrap());
        self.read()
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
}
