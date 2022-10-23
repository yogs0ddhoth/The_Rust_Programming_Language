mod list;
use list::List::{self, Cons, Nil};

mod my_box;
use my_box::MyBox;

/// Box<T> is a smart pointer that allows you to store data on the heap, 
/// leaving a pointer to the data on the stack 
fn new_box<T>(b: T) -> Box<T> {
    // When Box goes out of scope it will be dropped
    Box::new(b)
}

/// constructs a cons list from a vector of i32
fn cons_list(vec: Vec<i32>) -> List {
    let mut args = vec.iter();

    Cons(
        *args.next().unwrap(),
        Box::new(match args.next() {
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
    }

    #[test]
    fn it_works() {
        let b = new_box(5);
        println!("b = {}", b);

        assert_eq!(5, *b);
    }

    #[test]
    fn cons_list_works() {
        let list = cons_list(vec![1, 2, 3]);
        println!("List: {:?}", list);
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
