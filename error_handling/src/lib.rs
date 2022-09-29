use std::{fs::File, io::ErrorKind};

pub fn main() {
    /* Error handling with match expression */
    let greeting_file_result = File::open("hello.txt");
    let greeting_file1 = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file{:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    /* unwrap_or_else method error handling */
    let greeting_file2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
