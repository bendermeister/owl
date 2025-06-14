use crate::file_format::FileFormat;
use std::ffi::OsStr;
use std::path::PathBuf;

use crate::file::prelude::*;
use crate::time_stamp::TimeStamp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestPath {
    pub body: TestBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestBody {
    File(TestFile),
    Directory(TestDirectory),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestFile {
    pub path: PathBuf,
    pub body: String,
    pub modified: TimeStamp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestDirectory {
    pub path: PathBuf,
    pub body: Vec<TestPath>,
}

#[allow(refining_impl_trait)]
impl PathLike for TestPath {
    fn path(&self) -> PathBuf {
        match &self.body {
            TestBody::Directory(test_directory) => test_directory.path.clone(),
            TestBody::File(test_file) => test_file.path.clone(),
        }
    }

    fn is_dir(&self) -> bool {
        match &self.body {
            TestBody::File(_) => false,
            TestBody::Directory(_) => true,
        }
    }

    fn is_file(&self) -> bool {
        match &self.body {
            TestBody::File(_) => true,
            TestBody::Directory(_) => false,
        }
    }

    fn to_dir(self) -> Result<TestDirectory, TestFile> {
        match self.body {
            TestBody::File(test_file) => Err(test_file),
            TestBody::Directory(test_directory) => Ok(test_directory),
        }
    }

    fn to_file(self) -> Result<TestFile, TestDirectory> {
        match self.body {
            TestBody::File(test_file) => Ok(test_file),
            TestBody::Directory(test_directory) => Err(test_directory),
        }
    }
}

impl FileLike for TestFile {
    fn read(&mut self) -> String {
        self.body.clone()
    }

    fn modified(&self) -> TimeStamp {
        self.modified
    }

    fn extension(&self) -> Option<&OsStr> {
        self.path.extension()
    }

    fn file_format(&self) -> crate::file_format::FileFormat {
        self.extension()
            .map(|v| FileFormat::new(v))
            .unwrap_or(FileFormat::Unknown)
    }
}

#[allow(refining_impl_trait)]
impl DirectoryLike for TestDirectory {
    fn discover(self) -> Vec<TestFile> {
        let mut buf = Vec::new();

        self.body
            .into_iter()
            .for_each(|entry| match entry.to_dir() {
                Ok(dir) => buf.append(&mut dir.discover()),
                Err(file) => buf.push(file),
            });

        buf
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path() {
        let expected: PathBuf = "/this/is/a/path/file.md".into();
        let path = TestPath {
            body: TestBody::File(TestFile {
                path: expected.clone(),
                body: "Hello World".into(),
                modified: TimeStamp::from_ymd(2025, 12, 1).unwrap(),
            }),
        };

        assert_eq!(expected, path.path());

        let path = TestPath {
            body: TestBody::Directory(TestDirectory {
                path: expected.clone(),
                body: Vec::new(),
            }),
        };

        assert_eq!(expected, path.path());
    }

    #[test]
    fn test_is_dir() {
        let path = TestPath {
            body: TestBody::File(TestFile {
                path: "path/file.md".into(),
                body: "body".into(),
                modified: TimeStamp::now(),
            }),
        };

        assert!(!path.is_dir());

        let path = TestPath {
            body: TestBody::Directory(TestDirectory {
                path: "path/file.md".into(),
                body: Vec::new(),
            }),
        };

        assert!(path.is_dir());
    }

    #[test]
    fn test_is_file() {
        let path = TestPath {
            body: TestBody::File(TestFile {
                path: "path/file.md".into(),
                body: "body".into(),
                modified: TimeStamp::now(),
            }),
        };

        assert!(path.is_file());

        let path = TestPath {
            body: TestBody::Directory(TestDirectory {
                path: "path/file.md".into(),
                body: Vec::new(),
            }),
        };

        assert!(!path.is_file());
    }

    #[test]
    fn test_to_dir() {
        let file = TestFile {
            path: "path/file.md".into(),
            body: "body".into(),
            modified: TimeStamp::now(),
        };

        let path = TestPath {
            body: TestBody::File(file.clone()),
        };

        let got = match path.to_dir() {
            Ok(_) => panic!("this should be a file"),
            Err(file) => file,
        };

        assert_eq!(got, file);

        let dir = TestDirectory {
            path: "path/file.md".into(),
            body: Vec::new(),
        };
        let path = TestPath {
            body: TestBody::Directory(dir.clone()),
        };

        let got = match path.to_dir() {
            Ok(dir) => dir,
            Err(_) => panic!("this should be dir"),
        };
        assert_eq!(got, dir);
    }

    #[test]
    fn test_to_file() {
        let file = TestFile {
            path: "path/file.md".into(),
            body: "body".into(),
            modified: TimeStamp::now(),
        };

        let path = TestPath {
            body: TestBody::File(file.clone()),
        };

        let got = match path.to_file() {
            Ok(file) => file,
            Err(_) => panic!("this should be dir"),
        };

        assert_eq!(got, file);

        let dir = TestDirectory {
            path: "path/file.md".into(),
            body: Vec::new(),
        };
        let path = TestPath {
            body: TestBody::Directory(dir.clone()),
        };

        let got = match path.to_file() {
            Ok(_) => panic!("this should be dir"),
            Err(dir) => dir,
        };
        assert_eq!(got, dir);
    }

    #[test]
    fn test_read() {
        let mut file = TestFile {
            path: "this/is/some/path/file.md".into(),
            body: "this is a body".into(),
            modified: TimeStamp::now(),
        };

        assert_eq!(file.read(), "this is a body");
    }

    #[test]
    fn test_extension() {
        let file = TestFile {
            path: "this/is/some/path/file.md".into(),
            body: "this is a body".into(),
            modified: TimeStamp::now(),
        };

        assert_eq!(file.extension().unwrap(), "md");
    }

    #[test]
    fn test_modified() {
        let modified = TimeStamp::now();

        let file = TestFile {
            path: "this/is/some/path/file.md".into(),
            body: "this is a body".into(),
            modified: modified,
        };

        assert_eq!(file.modified(), modified);
    }

    #[test]
    fn test_discover() {
        let file0 = TestFile {
            path: "/deep/deep/file0.md".into(),
            body: "body0".into(),
            modified: TimeStamp::now(),
        };

        let file1 = TestFile {
            path: "/deep/deep/file1.md".into(),
            body: "body1".into(),
            modified: TimeStamp::now(),
        };

        let dir0 = TestDirectory {
            path: "/deep/deep/".into(),
            body: vec![
                TestPath {
                    body: TestBody::File(file0.clone()),
                },
                TestPath {
                    body: TestBody::File(file1.clone()),
                },
            ],
        };

        let file2 = TestFile {
            path: "/deep/file2.md".into(),
            body: "body1".into(),
            modified: TimeStamp::now(),
        };

        let file3 = TestFile {
            path: "/deep/file3.md".into(),
            body: "body3".into(),
            modified: TimeStamp::now(),
        };

        let dir1 = TestDirectory {
            path: "/deep/".into(),
            body: vec![
                TestPath {
                    body: TestBody::Directory(dir0.clone()),
                },
                TestPath {
                    body: TestBody::File(file2.clone()),
                },
                TestPath {
                    body: TestBody::File(file3.clone()),
                },
            ],
        };

        let mut expected = vec![file0, file1, file2, file3];
        let mut got = dir1.discover();

        expected.sort_by(|a, b| a.body.cmp(&b.body));
        got.sort_by(|a, b| a.body.cmp(&b.body));

        assert_eq!(expected, got);
    }
}
