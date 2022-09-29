use std::{
    error::Error,
    fs::{self, File},
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn Error>> {

    /* Error handling with match expression */
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file1 = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file{:?}", e),
    //         }
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     }
    // };

    /* unwrap_or_else method error handling */
    // let greeting_file2 = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    /* a more elegant error handling using expect() */
    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.text should be included in this project"); // expect() is preferable to unwrap()

    read_username_from_file().expect("hello.text should be included in this project");

    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

/* Propogate the error to the calling code */
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    // note the ? operator part of the method chain
    File::open("hello.txt")? // if Ok, return the value within
        .read_to_string(&mut username)?; // if Err, propogate error of type defined in function signature to calling code
    // Ok(username)

    /* the ? is roughly the same as:
     *  match username_file {
     *      Ok(file) => file,
     *      Err(e) => return Err(e),
     *  };
     */

    // see function for further use of the ? operator
    last_char_of_first_line(username.as_str());

    /* An even shorter way of doing this: */
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines() // return an iterator of the lines in a string (seperated by \n)
        .next()? // pop the top value in the iterator and move to next. if None, return early
        .chars() // return an iterator of the chars in the line
        .last() // return the last value in the iterator
}