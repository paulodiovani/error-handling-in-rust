use std::{
    fmt::Display,
    io::{Error, ErrorKind},
};

#[derive(Debug)]
pub enum FileError {
    NotFound,
    Empty,
    Forbidden,
    Unknown,
}

// Pretty print error messages
impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "File not found"),
            Self::Empty => write!(f, "File is empty"),
            Self::Forbidden => write!(f, "Missing permissions to read file"),
            Self::Unknown => write!(f, "Unknown error"),
        }
    }
}

// convert from io::Error
impl From<Error> for FileError {
    fn from(err: Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound => FileError::NotFound,
            ErrorKind::PermissionDenied => FileError::Forbidden,
            _ => FileError::Unknown,
        }
    }
}
