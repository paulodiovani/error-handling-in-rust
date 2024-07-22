mod errors;
mod file_contents;

use crate::file_contents::*;

fn main() {
    let filenames = vec![
        "sample-hello.txt",
        "sample-empty.txt",
        "sample-not-found.txt",
        "sample-forbidden.txt",
    ];

    for filename in filenames {
        let file_contents = read_file_and_unwrap(filename);
        // let file_contents = read_file_or_return_message(filename);
        // let file_contents = read_file_and_match(filename);
        // let file_contents = read_file(filename);

        match file_contents {
            Ok(contents) => println!("File contents: {}", contents),
            Err(err) => println!("Error reading file: {:?}", err),
        }
    }

}
