/* Associated Types allow trait method definitions to use placeholder types in their signatures */

pub trait Iterator {
    type Item; // associated type specified by the implementor
               // unlike generics which can be implemented for multiple types and require type annotations for methods,
               // associated types can only be implemented once, and require no type annotations, becoming part of the trait's contract
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}
