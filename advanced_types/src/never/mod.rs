/* 
the never type '!' has no values and so can be coerced into any other type
*/

fn _bar() -> ! { // the never type can indicate that a function never returns (i.e diverging functions)
    panic!("This call never returns.");
}

/* Some ! values:
    * continue
    * panic!
    * loops that never break
*/
