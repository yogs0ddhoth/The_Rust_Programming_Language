/// implementing Drop on a type will allow code to run on memory cleanup
/// (when the value goes out of scope and memory is deallocated, i.e. dropped)
pub struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: `{}`!", self.data);
    }
}

impl CustomSmartPointer {
    pub fn new(data: String) -> CustomSmartPointer {
        CustomSmartPointer { data }
    }
}
