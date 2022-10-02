// Lifetime annotations are necessary for references in structs
#[derive(Debug)]
pub struct ImportantExcerpt<'a> { // ImportantExcerpt can't outlive the reference in the 'part' field
    part: &'a str,
} // lifetimes are part of the type signature and have to be declared
impl<'a> ImportantExcerpt<'a> {
    // elision rule 1. used
    pub fn level(&self) -> i32 {
        3
    }

    // elision rules 1. & 3. used
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'"); // start 'a
    let i = ImportantExcerpt {                                          
        part: first_sentence,
    };
    println!("{}", i.part);
} // end 'a, first_sentence and i no longer valid