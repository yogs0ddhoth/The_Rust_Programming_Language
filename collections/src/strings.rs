// Strings are implemented as a wrapper around a vector of bytes**
pub fn print_strings() {
    let mut s = String::new(); // similar to the new() method on the Vector type

    let data = "STRINGS:\n";
    s = data.to_string(); // initialize String using the to_string() method
    println!("{}", s);

    let hello = vec![
        String::from("السلام عليكم"), // initialize using the from() method
        String::from("Dobrý den!"),
        String::from("Hello!"),
        String::from("!שָׁלוֹם"),
        String::from("नमस्ते!"),
        String::from("こんにちは!"),
        String::from("안녕하세요!"),
        String::from("你好!"),
        String::from("Olá!"),
        String::from("Здравствуйте!"),
        String::from("¡Hola!\n"),
    ];
    for s in hello {
        println!("{}", s);
    }

    s = String::from("H-");
    let s2 = "Hiya";

    s.push_str(s2); // append string slice without taking ownership
    s.push(' '); // append a single character to the String

    let s3 = String::from("Chuck");

    // '+' operator short for fn add(self, s: &str) -> String
    let s4 = s + &s3; // s moved, and no longer usable
                      // compiler uses deref coercion to turn &s3 into &s3[..] (&String -> &str)

    s = format!("{}-H-H-{}-{}-Ch-Chuuck.", s2, s4, s3); // format a string (doesn't take ownership)
    println!("{}", s);

    // the best way to operate on pieces of strings is to be use either characters or bytes***
    for c in "你好!".chars() {
        println!("\n{}:", c);
        for b in c.to_string().bytes() {
            println!("\t{}", b);
        }
    }
}
// ** with extended functionality
// *** some languages, like Devanagari, require an external crate to handle efficiently
