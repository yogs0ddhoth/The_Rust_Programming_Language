#[derive(Debug)]
pub enum List {
    Cons(
        i32,
        // using the Box type to store List makes memory allocation knowable, allowing recursive data structures
        Box<List>,
    ),
    Nil,
}
