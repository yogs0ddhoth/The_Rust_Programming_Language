/* The newtype pattern creates a new single field tuple type that serves as a wrapper for a type, that can implement traits the type cannot */
use std::{fmt, ops::Deref};

pub struct Wrapper(pub Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// implementing Deref is important so as to give access to the methods of Vec<T>
impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
