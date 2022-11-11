use std::slice;

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // safe rust will not allow borrowing from the same slice twice, even if the slice ranges borrowed don't overlap
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/* code written in other languages is declared within extern blocks and are always unsafe */
extern "C" {
    fn abs(input: i32) -> i32;
}
pub fn c_abs() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

/* extern can also allow other languages to call Rust functions */
#[no_mangle] // preserve function name
             // this function can be called from C
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
