/* Macros are a way of metaprogramming (writing code that writes code), that include:
    * declarative macros with macro_rules!
    * procedural macros:
        * custom #[derive] macros that specify code added with the derive attribute used on structs and enums
        * attribute-like macros that define custom attributes usable on any item
        * function-like macros that look like function calls but operate on the tokens specified as their argument

 * Macros are different from Rust functions in that:
    * a function signature must declare the number and type of parameters, while macros can take a variable number of parameters - i.e. println!()
    * macros are expanded upon before the compiler intereprets the code, and can so implement a trait, while functions are called at runtime
    * macros are by their nature more complex to implement and maintain
    * macros must be defined and brought into scope before calling them

 * Macros compare a value to patterns that are associated with particular code
*/
mod declarative;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct John;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declarative_works() {
        let vec = vecc!(1,2,3,4,5);
    }

    #[test]
    fn procedural_works() {
        John::hello_macro();
    }
}
