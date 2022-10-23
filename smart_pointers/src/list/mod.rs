use std::rc::Rc;
// i.e. Reference Count
#[derive(Debug)]
pub enum List {
    Cons(
        i32,
        // using the Rc type to store List makes memory allocation knowable, allowing recursive data structures
        // Rc<T> holds and tracks multiple ownership of the values it references
        Rc<List>,
    ),
    Nil,
}
