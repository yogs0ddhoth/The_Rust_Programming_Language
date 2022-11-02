/* Multiple Producer, Single Consumer */
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Rust implements channels to achieve message-sending concurrnecy
// A channel is made of a transmitter: Sender<T> and a receiver: Receiver<T>
pub fn single_sender() {
    // Create a new asynchronous channel, returning the sender/receiver halves
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hiya Chuck");

        // IMPORTANT: Ownership of val is sent along with the value
        tx.send(val) // will never block current thread
            .unwrap(); // in a real application, error handling would be necessary

        // println!("val is {}", val); will throw an error
    });

    let received = rx
        .recv() // will always block current thread and wait for a value
        // .try_recv // won't block current thread; returns Ok with a value and Err if there is none
        .unwrap();
    println!("Got: '{}' from thread\n", received);
}

pub fn multiple_senders() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // create a second sender to use
    thread::spawn(move || {
        let vals = vec![String::from("Hiya Chuck"), String::from("It's John")];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    /* Receiver<T> can be treated as an iterator */
    for received in rx {
        println!("Got: {}", received);
    }
}
