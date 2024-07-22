
use crate::errors::FileError;
use std::{fs::File, io::Read};

/// Reads a file and unwrap or panic!
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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