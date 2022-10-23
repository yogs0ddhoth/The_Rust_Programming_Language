mod custom_smart_pointer;
mod list;
mod my_box;

use custom_smart_pointer::CustomSmartPointer;
use list::List::{self, Cons, Nil};
use my_box::MyBox;
use std::rc::Rc;

/// Box<T> is a smart pointer that allows you to store data on the heap,
/// leaving a pointer to the data on the stack
fn new_box<T>(b: T) -> Box<T> {
    // When Box goes out of scope it will be dropped
    Box::new(b)
}

/// recursively constructs a cons list from a vector of i32
fn cons_list(vec: Vec<i32>) -> List {
    let mut args = vec.iter();

    Cons(
        *args.next().unwrap(),
        Rc::new(match args.next() {
            Some(_) => cons_list(vec[1..vec.len()].to_vec()),
            None => Nil,
        }),
    )
}

/// The compiler will do deref coersion on args, giving access to the stored value
///
/// All types that implement Deref will have it called
fn hiya(name: &str) {
    println!("Hiya {name}.");
}

// without the Deref trait, the compiler can only dereference &
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);

        // Variables are dropped in reverse order of their creation
        let _c = CustomSmartPointer::new(String::from("created first, dropped last - usually"));
        let _d = CustomSmartPointer::new(String::from("created last, dropped first"));
        println!("CustomSmartPointers created.");
        // c.drop() will not compile
        drop(_c); // allows dropping a value before it goes out of scope
        println!("CustomSmartPointer dropped before the end of main.");
    }

    #[test]
    fn it_works() {
        let b = new_box(5);
        println!("b = {}", b);

        assert_eq!(5, *b);
    }

    #[test]
    fn cons_list_works() {
        let a = Rc::new(cons_list(vec![5, 10]));
        println!("a: {:?}", a);
        println!("count after creating a = {}", Rc::strong_count(&a));
        // Rc will track another owner of its data with clone()*
        // * not a deep copy
        let b = Cons(3, Rc::clone(&a));
        println!("b: {:?}", b);
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("c: {:?}", c);
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
        // data will not be cleaned until all references are dropped
    }

    #[test]
    fn hello() {
        let chuck = MyBox::new(String::from("Chuck"));
        /* Rust will do deref coercion:
         *  - From &T to &U when T: Deref<Target=U>
         *  - From &mut T to &mut U when T: DerefMut<Target=U>
         *  - From &mut T to &U when T: Deref<Target=U>
         *  -- Immutable references will never deref to mutable
         */
        hiya(&chuck);
    }
}
