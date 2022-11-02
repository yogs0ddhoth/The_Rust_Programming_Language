mod message_passing;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

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
}
