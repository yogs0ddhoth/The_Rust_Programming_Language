use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
/*
 * Reference Cycle:
 */
fn ref_cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // modify a.tail()'s dereferenced mutable value so that it points to b, instead of Nil
    // this will create a reference cycle that will overflow the stack
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail());
}
fn main() {
    // ref_cycle();
    node_tree();
}

#[derive(Debug)]
struct Node {
    value: &'static str,
    // the combination of weak and strong pointers avoids a reference cycle
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn node_tree() {
    // child node
    let leaf = Rc::new(Node {
        value: "Chuck",
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}\n",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        // parent node
        let branch = Rc::new(Node {
            value: "Hiya",
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]), // increase reference_count
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // Rc::downgrade(&Rc<T>) returns a Weak<T> pointer that does not increase the strong_count

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}\n",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("{:?}", leaf.parent.borrow().upgrade()); // Weak<T>::upgrade() returns Option<Rc<T>> if the value still exists
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
