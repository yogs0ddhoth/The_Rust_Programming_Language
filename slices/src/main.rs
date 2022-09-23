fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&mut s);

    // s.clear(); <- will cause error!

    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}