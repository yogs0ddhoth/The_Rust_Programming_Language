/* LIFETIME ELISIONS 
 *
 * compiler will check three rules to infer lifetimes when there aren't explicit annotations:
 * 
 * 1. compiler assigns a lifetime param to each param that's a ref:
 *      fn foo<'a, 'b ...>(x: &'a i32, y: &'b i32 ...)
 * 
 * 2. if there is exactly one input lifetime param, that lifetime is assigned to all return lifetime params:
 *      fn foo<'a>(x: &'a i32) -> &'a i32
 * 
 * 3. if there are multiple input lifetime params, but one of them is &self/&mut self, the lifetime of self is assigned to the output lifetime params:
 *      fn foo<'a, 'b>(x: &'a i32, &'b self) -> &'b i32
 * 
 * Example: fn first_word(s: &str) -> &str
 *  compiler:
 * 1.       fn first_word<'a>(s: &'a str) -> &str
 * 
 * 2.       fn first_word<'a>(s: &'a str) -> &'a str
 * 
 * 3.       no &self/&mut self, third rule not used
 * -----------------------------------------------------
 * Example: fn longest(x: &str, y: &str) -> &str
 *  compiler:
 * 1.       fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str
 * 
 * 2. 3.    multiple refs, no &self/&mut self, second/third rules not used
 * 
 *          lifetime of return value unsure. compiler throws an error requiring it be annotated - see lifetimes.rs
 * */