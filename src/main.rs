mod errors;
mod file_contents;

use crate::file_contents::*;

fn main() {
    let filename = String::from("sample-hello.txt");
    // let filename = String::from("sample-empty.txt");
    // let filename = String::from("sample-not-found.txt");
    // let filename = String::from("sample-forbidden.txt");

    // let file_contents = read_file_and_unwrap(filename);
    // let file_contents = read_file_and_match(filename);
    let file_contents = read_file(filename);

    match file_contents {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => println!("Error reading file: {:?}", err),
    }
}
