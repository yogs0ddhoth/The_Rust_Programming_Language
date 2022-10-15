mod list;
use list::List::{self, Cons, Nil};
/* Box<T> is a smart pointer that allows you to store data on the heap, leaving a pointer to the data on the stack */
fn new_box<T>(b: T) -> Box<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let b = new_box(5);
        println!("b = {}", b);

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        println!("List: {:?}", list);

        println!("List: {:?}", cons_list(vec![1, 2, 3]));
    }
}
