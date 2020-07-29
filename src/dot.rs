use proc_macro2::TokenStream;
use quote::{quote,format_ident};

pub fn quote_itemfn(f: syn::ItemFn) -> TokenStream {
   let fi = f.sig.ident.clone();
   let fctor = format_ident!("{}_ctor", fi ,span=fi.span());
   let fi_s = syn::LitStr::new(&fi.to_string(), fi.span());
   quote! {
      #f
      #[ctor::ctor]
      fn #fctor() {
         println!("{} {} = {}", std::module_path!(), #fi_s, #fi());
      }
   }
}
