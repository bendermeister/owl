use std::ffi::OsStr;

pub enum FileFormat {
    Unknown,
    Markdown,
    Typst,
}

impl FileFormat {
    pub fn new(extension: &OsStr) -> Self {
        match extension.to_str() {
            Some("md") => Self::Markdown,
            Some("typ") => Self::Typst,
            _ => Self::Unknown,
        }
    }
}
