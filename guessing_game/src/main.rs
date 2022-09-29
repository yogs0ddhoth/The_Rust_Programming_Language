use rand::Rng;
use std::{cmp::Ordering, io};

use guessing_game::Guess;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Ok(g) => g.value(),
                Err(msg) => {
                    println!("{msg}");
                    continue;
                }
            },
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
