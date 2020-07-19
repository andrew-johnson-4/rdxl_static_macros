#![recursion_limit = "128"]
#![crate_type = "proc-macro"]

use proc_macro::{TokenStream};
use quote::{quote};

#[proc_macro_attribute]
pub fn dot(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("dot attr: {}", attr);
    println!("dot item: {}", item);

    let expanded = quote! {
        {
            let mut stream = String::new();
            stream
        }
    };

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
