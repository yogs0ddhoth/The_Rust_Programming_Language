use std::{
    rc::Rc, 
    cell::RefCell
};

#[derive(Debug)]
pub enum List {
    Cons(
        /**
         * Wrapping RefCell<T> in Rc<T> allows for multiple mutable pointers
         */
        Rc<RefCell<i32>>,

        /**
         * Rc<T> holds and tracks multiple ownership of the values it references. 
         * Using the Rc type to store List makes memory allocation knowable, allowing recursive data structures
         */
        Rc<List>,
    ),
    Nil,
}
