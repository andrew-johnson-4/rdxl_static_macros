#![recursion_limit = "128"]
#![crate_type = "proc-macro"]

use proc_macro::{TokenStream};
use syn::{parse_macro_input};
use quote::{quote};
use rdxl_internals::xhtml;

#[proc_macro_attribute]
pub fn dot(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as syn::ItemFn);

    let expanded = quote! {
       #f
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn dot_template(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let f = parse_macro_input!(item as syn::ItemFn);

    let expanded = quote! {
       #f
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn dot_html(input: TokenStream) -> TokenStream {
    let c = parse_macro_input!(input as xhtml::XhtmlCrumb);

    let expanded = quote! {
       {
          let mut stream = String::new();
          #c
          DotHtml {
             content: stream
          }
       }
    };

    TokenStream::from(expanded)
}
