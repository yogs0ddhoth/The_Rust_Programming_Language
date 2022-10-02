pub mod structs;
use std::fmt::Display;
/* LIFETIMES
 * The lifetimemes of r and x, as 'a and 'b, respectively 
 * fn main() {
 *     let r;                // ---------+-- 'a
 *                           //          |
 *     {                     //          |
 *         let x = 5;        // -+-- 'b  |
 *         r = &x;           //  |       |
 *     }                     // -+       | // x no longer exists
 *                           //          | // r no longer has a value
 *     println!("r: {}", r); //          | 
 * }                         // ---------+
 * this code will cause an error because 'b does not live long enough
 * 
 * fn main() { // r will always be vaild while x is valid
 *     let x = 5;            // ----------+-- 'b
 *                           //           |
 *     let r = &x;           // --+-- 'a  |
 *                           //   |       |
 *     println!("r: {}", r); //   |       |
 *                           // --+       | 
 * }                         // ----------+
 * this will compile
 * */
/* Lifetime Annotation:
 * &i32        // a reference
 * &'a i32     // a reference with an explicit lifetime 'a
 * &'a mut i32 // a mutable reference with an explicit lifetime 'a
 * 
 * annotations describe the relationships of lifetimes
*/


pub fn longest_with_an_announcement<'a, T>( // 'a: lifetime, T: see where
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str   // return shares lifetime with params
where
    T: Display // T: Display: trait
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn main() {
    let string1 = String::from("Hiya Chuck");
    {   
        // string2 has the shorter lifetime, and will be the reference lifetime
        let string2 = String::from("It's John");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}\n", result); 
    }   // end of string2 and result lifetime (string1 lifetime extends past this scope), completing the contract in longest()
}

/* Lifetime Parameters:
 * lifetime annotations become part of the function contract
 * */
// the returned ref will be valid as long as both the parameters are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// return value will be valid for the lifetime of x
// i.e. only lifetime of x is in relationship to the return lifetime
pub fn single_lifetime<'a>(x: &'a str, _y: &str) -> &'a str { 
    // 'static lifetimes can last the duration of the program
    let _s: &'static str = "I have a static lifetime."; // this reference will always be available in the program's binary
    x 
}

