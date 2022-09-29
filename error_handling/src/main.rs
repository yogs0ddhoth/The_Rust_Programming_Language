use error_handling::main as greeting_file;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, Read},
    net::IpAddr,
};
/* ERROR HANDLING GUIDELINES:
 *
 * return a Result when failure is an anticipated possibility
 *
 * code should panic to prevent a bad state:
 *
 * * when some assumption, guarantee, contract, or invariant has been broken:
 * ** when invalid, contradictory, or missing values are passsed
 * ** something unexpected happens, like data in the wrong format
 * ** code after this point relies on not being in a bad state
 * ** there's no good way to encode this information in types
 *
 * * when calling external code that returns a bad state
 */

fn main() -> Result<(), Box<dyn Error>> {
    /* error handling with match expressions, unwrap_or_else, see lib.rs */
    greeting_file();

    /* a more elegant error handling using expect() */
    let greeting_file =
        File::open("hello.txt").expect("hello.text should be included in this project"); // expect() is preferable to unwrap()

    read_username_from_file().expect("hello.text should be included in this project");

    /* The simplest way to propogate errors */
    let greeting_file = File::open("hello.txt")?;

    let home: IpAddr = "127.0.0.1"
        .parse() // parse returns a Result
        // .expect("Hardcoded IP address should be valid"); // expect() is unnessecary in this situation because the IP address is hardcoded
        .unwrap(); // because this code can logically never return an Err, unwrap() is a better way to handle the Result

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
