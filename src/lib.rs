#![recursion_limit = "128"]
#![crate_type = "proc-macro"]

use proc_macro::{TokenStream};
use syn::{parse_macro_input};
use quote::{quote};
mod dot_template;
use dot_template::DotTemplate;
mod dot_html;
use dot_html::DotHtmlInvocation;

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
    let f = parse_macro_input!(item as DotTemplate);

    let expanded = quote! {
       #f
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn dot_html(input: TokenStream) -> TokenStream {
    let c = parse_macro_input!(input as DotHtmlInvocation);

    let expanded = quote! {
       #c
    };

    TokenStream::from(expanded)
}
