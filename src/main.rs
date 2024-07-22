#[allow(dead_code)]
mod file_contents {
    use std::{
        fmt::Display,
        fs::File,
        io::{Error, ErrorKind, Read},
    };

    pub enum FileError {
        NotFound,
        Empty,
        Forbidden,
        Unknown,
    }

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

    impl From<Error> for FileError {
        fn from(err: Error) -> Self {
            match err.kind() {
                ErrorKind::NotFound => FileError::NotFound,
                ErrorKind::PermissionDenied => FileError::Forbidden,
                _ => FileError::Unknown,
            }
        }
    }

    /// Reads a file and print contents or error message
    pub fn read_file(filename: String) -> Result<String, FileError> {
        let mut file = File::open(filename)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        if content.is_empty() {
            Err(FileError::Empty)
        } else {
            Ok(content)
        }
    }
}

use file_contents::*;

fn main() {
    let filename = String::from("sample-hello.txt");
    // let filename = String::from("sample-empty.txt");
    // let filename = String::from("sample-not-found.txt");
    // let filename = String::from("sample-forbidden.txt");

    match read_file(filename) {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => println!("Error reading file: {}", err),
    }
}
