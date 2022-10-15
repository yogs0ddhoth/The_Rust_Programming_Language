use add_one;
use add_two;
use rand;

fn main() {
    let num = add_two::add_two(rand::random::<i32>());
    println!("Hiya Chuck, {num} plus one is {}!", add_one::add_one(num));
}
