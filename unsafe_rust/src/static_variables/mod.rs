// static variables are rust's version of global variables, and can only store variables with 'static lifetimes
// immutable variables are always safe
// unlike constants, static variables have a fixed address in memory and can be mutable

static mut COUNTER: u32 = 0;

// mutable global variables are difficult to implement without data races, and so are unsafe
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn works() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
