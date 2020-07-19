#![recursion_limit = "128"]
#![crate_type = "proc-macro"]

use proc_macro::{TokenStream};
use quote::{quote};

#[proc_macro]
pub fn dot_html(input: TokenStream) -> TokenStream {
    println!("{}", input);

    let expanded = quote! {
        {
            let mut stream = String::new();
            stream
        }
    };

    TokenStream::from(expanded)
}
