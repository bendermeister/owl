use std::ffi::OsStr;
use std::path::Path;

pub enum FileFormat {
    Unknown,
    Markdown,
    Typst,

    C,
    CPP,
    Rust, 
    Go,
    Java,
    JavaScript,
    TypeScript,
}

impl FileFormat {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        path.as_ref()
            .extension()
            .map(|ext| Self::from_extension(ext))
            .unwrap_or(FileFormat::Unknown)
    }

    pub fn from_extension(extension: &OsStr) -> Self {
        match extension.to_str() {
            Some("md") => Self::Markdown,
            Some("typ") => Self::Typst,

            Some("c") => Self::C,
            Some("h") => Self::C,

            Some("cpp") => Self::CPP,
            Some("cc") => Self::CPP,
            Some("hh") => Self::CPP,

            Some("rs") => Self::Rust,
            Some("go") => Self::Go,
            Some("java") => Self::Java,
            Some("js") => Self::JavaScript,
            Some("ts") => Self::TypeScript,

            _ => Self::Unknown,
        }
    }

    pub fn is_unknown(&self) -> bool {
        match self {
            FileFormat::Unknown => true,
            _ => false,
        }
    }

    pub fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}

impl<P> From<P> for FileFormat
where
    P: AsRef<Path>,
{
    fn from(path: P) -> Self {
        Self::new(path)
    }
}
