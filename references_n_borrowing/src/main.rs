fn main() {
    {
        let mut s = String::from("World");

        // &s1 references (points to) s
        let len = calculate_len(&s); // function borrows s through the reference

        println!("The length of '{}' is {}.", s, len);

        /* Multiple immutable references to the same value can exist in the same scope*/
        let s1 = &s;
        let s2 = &s;

        println!("Hello {} {} world world world world world wor-", s1, s2);

        {
            /* But a mutable reference MUST exist by itself within a scope */
            let s3 = &mut s;
            s3.push_str("-");

            /* NOTE: a reference's scope starts when it is introduced and continues through the last time it is used */

            let s4 = &mut s;
            s4.push_str("-");

            println!("{}", s4);
        }
    }
    
    {
        let mut s = String::from("Hello");
        change(&mut s); // NOTE: only one mutable reference to a value can exist at a time
        println!("{s}");
    }

}

fn calculate_len(s: &String) -> usize { // accepts a reference a String (which owns a growable string stored on the heap)
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}