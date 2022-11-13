use proc_macro::TokenStream;
use quote::quote; // turn sun data structures back into Rust code
use syn; // parse Rust code from a string into a data structure that can be operated on

#[proc_macro_derive(HelloMacro)] // indicate that the function will be called when #[derive(HelloMacro)]type
// this recipe is ubiquitous
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input)
        .unwrap(); // in production code, more specific error messages are needed

    impl_hello_macro(&ast)
}

// Build the trait implementation
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name { // # templates in the variable (name in this case)
            fn hello_macro() {
                println!(
                    "Hiya Chuck, it's {}.", 
                    stringify!(#name) // does what it says on the tin
                );
            }
        }
    };
    gen.into()
}