mod memory_sharing;
mod message_passing;

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::{thread, time::Duration, sync::{Arc, Mutex}};

    #[test]
    fn basic_threading() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("iteration {} from spawned thread", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle
            .join() // block the current thread, and wait for the associated thread to finish
            .unwrap();

        for i in 1..5 {
            println!("iteration {} from main thread", i);
            thread::sleep(Duration::from_millis(1));
        }
        // when the main thread of the program completes, all spawned threads are shut down whether or not they have finished running
    }

    #[test]
    fn moving_data() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(
            /* force the closure to take ownership of scope values */
            move || {
                println!("Here's a vector: {:?}", v);
            },
        );
        // v now cannot be used
        handle.join().unwrap();
    }

    #[test]
    fn message_passing_concurrency() {
        message_passing::single_sender();
        message_passing::multiple_senders();
    }

    #[test]
    fn shared_memory_concurrency() {
        memory_sharing::main();
        memory_sharing::mutual_exclusion();
    }

    #[test]
    pub fn deadlock() {
        /* A deadlock occurs when an operation needs to lock two resources, and two threads have each acquired one of the locks 
         * This can be mitigated with careful management of Mutexes
         */
        let m1 = Arc::new(Mutex::new(String::from("Hiya")));
        let m2 = Arc::new(Mutex::new(String::from("Chuck")));
    
        let mut handles = vec![];
        {
            let m1_clone = Arc::clone(&m1);
            handles.push(thread::spawn(move || {
                let guard = m1_clone.lock().unwrap();
                println!("{guard}");

                drop(guard); // comment this out to create a deadlock that will freeze the program
            }));
        }
    
        {
            let m2_clone = Arc::clone(&m2);
            handles.push(thread::spawn(move || {
                let guard = m2_clone.lock().unwrap();
                println!("{guard}");

                drop(guard); // comment this out to create a deadlock that will freeze the program
            }));
        }

        {
            let m1_clone = Arc::clone(&m1);
            let m2_clone = Arc::clone(&m2);
            handles.push(thread::spawn(move || {
                let mut lock_m1 = m1_clone.lock().unwrap();
                *lock_m1 = String::from("It's");

                let mut lock_m2 = m2_clone.lock().unwrap();
                *lock_m2 = String::from("John");

                println!("{}, {}", lock_m1, lock_m2);
            }));
        }
    
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
