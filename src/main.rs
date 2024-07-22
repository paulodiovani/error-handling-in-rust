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

    /// Reads a file and unwrap or panic!
    pub fn read_file_and_unwrap(filename: String) -> Result<String, ()> {
        let mut file = File::open(filename).unwrap();

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        if content.is_empty() {
            panic!("File is empty");
        }

        Ok(content)
    }

    /// Reads a file and print contents of error message (the verbose way)
    pub fn read_file_and_match(filename: String) -> Result<String, FileError> {
        let file = File::open(filename);

        match file {
            Ok(mut file) => {
                let mut content = String::new();
                match file.read_to_string(&mut content) {
                    Ok(_) => {
                        if content.is_empty() {
                            Err(FileError::Empty)
                        } else {
                            Ok(content)
                        }
                    }
                    Err(_) => Err(FileError::Unknown),
                }
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Final: Reads a file and print contents or error message
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
    // let filename = String::from("sample-hello.txt");
    let filename = String::from("sample-empty.txt");
    // let filename = String::from("sample-not-found.txt");
    // let filename = String::from("sample-forbidden.txt");

    let file_contents = read_file_and_unwrap(filename);
    // let file_contents = read_file_and_match(filename);
    // let file_contents = read_file(filename);

    match file_contents {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => println!("Error reading file: {:?}", err),
    }
}
