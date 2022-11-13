/* Procedural Macros act similar to functions, accepting code as input, operating on that code, and prodcing code:
 * There are 3 types of procedural macros:
    * custom derive
    * attribute-like
    * function-like
 * definitions for procedural macros must reside in their own crate with a special crate type - SEE CRATE DEPENDENCIES
*/

// attribute-like macros generate new attributes:
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// becomes:
#[route(GET, "/")]
fn index() {}

/* function-like macros are like functions that follow macro_rules! and can take an unknown number of arguments */
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}

// becomes:
sql!(SELECT * FROM posts WHERE id=1);