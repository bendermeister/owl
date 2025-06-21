use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    FailedToParse(usize),
    FailedToFindVar(String),
    Other(Box<dyn std::error::Error>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FailedToParse(line) => write!(f, "Error: Failed parsing in line: {}", line),
            Error::FailedToFindVar(var) => {
                write!(f, "Error: Failed to find environment variable: {}", var)
            }
            Error::Other(error) => write!(f, "Error: {}", error),
        }
    }
}

impl std::error::Error for Error {}
