use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Unknown,
    Markdown,
    Typst,
    Latex,

    Java,
    JavaScript,
    Go,
    C,
    CPP,
    Rust,
    Typescript,
    Shell,

    CSharp,
    Python,

    Nix,
}

impl Format {
    pub fn is_unknown(&self) -> bool {
        *self == Format::Unknown
    }

    pub fn is_known(&self) -> bool {
        !self.is_unknown()
    }

    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path: &Path = path.as_ref();
        match path.extension().map(|ext| ext.to_str()).flatten() {
            Some("md") => Self::Markdown,
            Some("typ") => Self::Typst,
            Some("java") => Self::Java,
            Some("js") => Self::JavaScript,
            Some("go") => Self::Go,
            Some("cpp") => Self::CPP,
            Some("hpp") => Self::CPP,
            Some("cc") => Self::CPP,
            Some("cxx") => Self::CPP,
            Some("c") => Self::C,
            Some("h") => Self::C,
            Some("rs") => Self::Rust,
            Some("ts") => Self::Typescript,
            Some("sh") => Self::Shell,
            Some("cs") => Self::CSharp,
            Some("py") => Self::Python,
            Some("nix") => Self::Nix,
            Some("latex") => Self::Latex,
            Some("tex") => Self::Latex,
            _ => Self::Unknown,
        }
    }
}
