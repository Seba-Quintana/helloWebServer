use proc_macro::TokenStream;
use quote::quote;
use syn;

// hello_macro_derive will be called when we invoke #[derive(HelloMacro)]
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    // here unwrap is used because we need the funtion to return 
    // a TokenStream (it would be better to panic with expect or 
    // panic!, to give a more informative error message)
    let ast = syn::parse(input).unwrap();
    // parse receives a TokenStream and returns a syn::DeriveInput, 
    // which looks like this: *

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// * syn::DeriveInput is a syntax tree data structure that lools 
// like this:
// DeriveInput {
//     // --snip--
//
//     ident: Ident {
//         ident: "Pancakes",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // assigns the identifier to name
    // it has the name of the struct we're deriving HelloMacro for
    let name = &ast.ident;
    // quote! macro lets you define the rust code you want to return
    let gen = quote! {
        // quote! macro lets you use #name, which will be replaced 
        // with the variable name we defined above (that has the name 
        // of the struct we're deriving HelloMacro for)
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! macro will turn the name of the struct 
                // into a string literal
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // into method converts the quote! into a TokenStream by 
    // consuming the intermediate representation
    gen.into()
}

