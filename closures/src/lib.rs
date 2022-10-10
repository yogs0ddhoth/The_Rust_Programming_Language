/* Closures are anonymous functions that capture values from the scope in which they're defined */
pub mod store;
pub mod shapes;
#[cfg(test)]
mod test {
    use crate::{shapes::Rectangle, store::{Inventory, ShirtColor}};

    use std::thread;

    #[test]
    fn shirt_company() {
        let store = Inventory::new(
            vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
        );
    
        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}", 
            user_pref1, giveaway1
        );
    
        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }

    #[test]
    fn main() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);
        // no other borrows can occur
        borrows_mutably();
        println!("After calling closure: {:?}", list);

        thread::spawn( // span a new thread, ownership of list is transfered for the duration of the thread
            move || println!("From thread: {:?}", list)
        )
        .join()
        .unwrap();

        let mut list = vec![
            Rectangle { width: 10, height: 1},
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 }
        ];
        let mut num_sort_operations = 0;

        list.sort_by_key(
            /* closure takes iterated value */
            |r| {
                // num_sort_operations is moved into the closure and mutated
                num_sort_operations += 1;
                // width field from param is returned
                r.width
            }
        );
        println!("{:#?}, sorted in {num_sort_operations} operations", list)
    }
}
