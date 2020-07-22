use quote::{format_ident, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{ItemFn,FnArg,Pat};
use inflector::cases::classcase::to_class_case;

pub struct DotTemplate {
   f: ItemFn
}

impl Parse for DotTemplate {
    fn parse(input: ParseStream) -> Result<Self> {
        let f: ItemFn = input.parse()?;

        Ok(DotTemplate {
            f: f,
        })
    }
}

impl ToTokens for DotTemplate {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      let ref f = self.f;
      let ref fid = f.sig.ident;
      let tid = to_class_case(&format!("{}Template", f.sig.ident));
      let tid = format_ident!("{}", tid, span=f.sig.ident.span());
      let mut pfs = Vec::new();
      let mut sfs = Vec::new();
      let mut tfs = Vec::new();
      for i in f.sig.inputs.iter() {
         if let FnArg::Typed(pt) = i {
            if let Pat::Ident(ref pi) = *pt.pat {
               pfs.push(pi.ident.clone());
               sfs.push(format_ident!("set_{}", pi.ident, span=pi.ident.span()));
               tfs.push(pt.ty.clone());
            }
         }
      }

      quote_spanned!(syn::spanned::Spanned::span(f)=>
         struct #tid {
            #(
               #pfs: #tfs,
            )*
         }
         impl #tid {
            pub fn new() -> #tid {
               #tid {
                  #(#pfs : std::default::Default::default(), )*
               }
            }
            #(
            pub fn #sfs(mut self, #pfs: String) -> #tid {
               self.#pfs = #pfs;
               self
            }
            )*
            pub fn build(&self) -> String {
               #fid( #(self.#pfs.clone()),* )
            }
         }
         #f
      ).to_tokens(tokens);    
   }
}
