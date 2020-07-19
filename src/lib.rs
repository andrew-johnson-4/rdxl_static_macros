#![recursion_limit = "128"]
#![crate_type = "proc-macro"]

use proc_macro::{TokenStream};
use syn::{parse_macro_input};
use quote::{quote};

#[proc_macro_attribute]
pub fn dot(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as syn::ItemFn);

    let expanded = quote! {
       #f
    };

    println!("{}", expanded);

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn dot_html(input: TokenStream) -> TokenStream {
    println!("dot_html: {}", input);

    let expanded = quote! {
        {
            let mut stream = String::new();
            stream
        }
    };

    TokenStream::from(expanded)
}
