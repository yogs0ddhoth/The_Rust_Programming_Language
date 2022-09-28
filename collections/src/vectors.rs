enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
// Vectors are growable collections of the same type of value
pub fn print_vectors() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2]; // create a mutable instance of a vector of i32
    v.push(3);
    v.append(&mut vec![4, 5]);

    println!("{:?}", v);

    // will cause panic if element does not exist
    let first = &v[0];
    println!("The first element is {}", first);

    // best use get method when unsure if element exists
    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {}\n", third),
        None => println!("There is no third element.\n"),
    }

    /* Examples of iteration: */
    // immutable
    for i in &v {
        println!("{}", i)
    }
    // mutable
    for i in &mut v {
        *i += 5;
        println!("{}", i)
    }

    // Use enums to create vectors with multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("\nrow:");
    for i in &row {
        match i {
            SpreadsheetCell::Int(int) => println!("int: &i32 = {}", int),
            SpreadsheetCell::Text(txt) => println!("text: {}", txt),
            SpreadsheetCell::Float(flt) => println!("float: &f64 = {}\n", flt),
        };
    }
} // all vectors are dropped from the heap when they leave scope
