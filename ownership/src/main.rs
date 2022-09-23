fn main() {
    /* values of a known size are stored on the stack */
    let x = 5;
    
    make_copy(x); // x copied into function

    /* values of variable size are stored on the heap */
    {   // begin scope. no values declared.

        // s1 comes into scope
        let s1 = String::from("Hello"); // stored on the stack, containing metadata and a pointer to a growable string stored on the heap
        let s2 = s1.clone(); // s1 deeply (ptr, len, capacity, & data) copied to s2. both s1 and s2 can be used

        // s1 moved (ptr, len, capacity) to s3, data remains unchanged in the heap.
        let mut s3 = s1; // s1 is now invalidated and can't be used
        s3.push_str(", world!");
    
        println!("{}, world! Also: {}\n", s2, s3);

        take_ownership(s3); // s3 moved into function
        // s3 dropped once function completes

    }   // scope over, s2 automatically dropped from the heap

    let s1 = give_ownership(); // return moved to s1
    let s2 = String::from("world"); // s2 comes into scope
    let (mut s3, len) = calculate_len(s2); // s2 moved into function. ownership returned and moved to s3 (along with len)

    println!("The length of '{}' is {}.", s3, len);

    s3 = "Chuck".to_string();
    
    let s4 = take_and_give_ownership(s3); // s3 moved into function
    // ownership returned and moved to s4

    println!("By the way: {}, {}.\n", s1, s4);

} // s1, s4 dropped form heap

fn take_ownership(str: String) { // str comes into scope
    println!("Finally: {}\n", str);
} // str goes out of scope and is dropped from the heap

fn make_copy(int: i32) { // int comes into scope
    println!("{}", int);
} // scope over. int is not stored on the heap, so nothing special happens 

fn give_ownership() -> String { // returns ownership of a string stored on heap
    let str = String::from("Hiya"); // str comes into scope
    str // str returned and moved to calling scope
}

fn take_and_give_ownership(str: String) -> String { // str comes into scope
    str // returned and moved to calling scope
}

fn calculate_len(str: String) -> (String, usize) {
    let length = str.len(); // len is a property of str: String

    (str, length) // ownership and length is returned as a tuple
}