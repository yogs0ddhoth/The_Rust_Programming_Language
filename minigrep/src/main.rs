use std::{env, process};

use minigrep::Config;

fn main() {
    println!("\nHiya, Chuck.");
    let args: Vec<String> = env::args().collect(); // collect args from the cl - will panic if any arguments contain invalid unicode
                                                   // args[0] will be the binary - "target\debug\minigrep.exe"
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: '{err}'");
        process::exit(1);
    });
    println!(
        "Searching for '{}' in file: '{}'{}\n",
        config.query, config.file_path, if config.ignore_case {" --ignore-case"} else {""}
    );
    if let Err(e) = minigrep::run(config) {
        // run main functionality from lib.rs while handling possible errors
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
