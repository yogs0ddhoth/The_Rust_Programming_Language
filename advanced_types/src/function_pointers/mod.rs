/* functions coerce to type fn or a function pointer */
pub fn add_one(x: i32) -> i32 { x + 1 }

pub fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), and so can be passed as arguments for a closure parameter
// NOTE: C does not have closures, and so external C functions can only accept functions as arguments
pub enum Status {
    Value(u32),
    Stop,
}
/* Because closures are represented by traits, they can only be returned as a trait object */

pub fn returns_to_string() -> Box<dyn Fn(Status) -> String> {
    Box::new(|i| {
        match i {
            Status::Value(x) => x.to_string(),
            Status::Stop => panic!("can't coerce to string!")
        }
    })
}