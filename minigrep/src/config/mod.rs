use std::env;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // first value will be the binary - "target\debug\minigrep.exe".
        args.next();

        /* consume the rest of the iterator */
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("You need more arguments, Chuck."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("You need to give a file path, Chuck."),
        };

        let ignore_case = match args.next() {
            Some(arg) => {
                if arg == "--ignore-case" {
                    true
                } else {
                    env::var("IGNORE_CASE").is_ok()
                }
            }

            None => env::var("IGNORE_CASE").is_ok(),
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
