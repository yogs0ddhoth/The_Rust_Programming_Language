fn main() {
    let mut number = 3;

    /* 'if else' statement */
    if number < 5 { // conditions do not automatically convert to booleans
 /* if number { // will throw an error */ 
        println!("condition was true")
    } else {
        println!("condition was false")
    }

    number = 6;

    /* 'if else' statement with 'else if' expressions*/
    if number % 4 == 0 {
        print!("number is divisible by 4 (and 2)")
    } else if number % 3 == 0 {
        println!("number is divisible by 3")
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("numebr is not divisible by 4, 3, or 2")
    }

    let condition = true;
    number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {number}");

 }