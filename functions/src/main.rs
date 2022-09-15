/* Statement */
fn main() {
    // Statement binding the return of an Expression
    let y = {
        // Statement binding the return of an Expression
        let x = five();
        
        plus_one(x)
    };
    println!("The value of y is: {y}");

    print_labeled_measurement(5, 'h');
}

/* Expression */
fn five() -> i32 {
    5
}

/* Expression */
fn plus_one(x: i32) -> i32 {
    // 'return' keywords allows the use of a semicolon at the end of the expression
   return x + 1;
}

/* Expression */
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}