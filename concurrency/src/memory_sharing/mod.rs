use std::{thread, sync::{Arc, Mutex}, time::Duration};

pub fn main() {
    let m = Mutex::new(5);
    println!("m = {:?}", m);
    { /* MutexGuard implements Deref and Drop */
      let mut num = m
          .lock() // aquire the mutex lock to manipulate data
          .unwrap(); // panic if call fails
      *num = 6; // the data can be dereferenced and mannipulated

      // lock() blocks the thread
    } // the lock is released when the variable goes out of scope and is dropped

    println!("now m = {:?}", m);
}

pub fn mutual_exclusion() {
    /* Atomics are primitive types that are concurrency safe**
     * Arc<T> has the same API as Rc<T> 
     * ** at a performance cost compared to single thread primitives
     */
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for i in 1..11 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            /* IMPORTANT! Mutex<T> provides interior mutability and so runs the risk of Memory leaks 
             */ 
            let mut num = counter.lock().unwrap();
            thread::sleep(Duration::from_millis(1000/i));
            *num += 1;

            
        });
        handles.push(handle);


    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Mutex accessed and mutated {} times", *counter.lock().unwrap());
}
