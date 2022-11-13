// the alias Kilometers is synonymous to i32 and will be treated as such by the compiler
pub type Kilometers = i32;

// the main use case is the reduce repetition
pub type Thunk = Box<dyn Fn() + Send + 'static>;

// example from the standard library:
type Result<T> = std::result::Result<T, std::io::Error>;