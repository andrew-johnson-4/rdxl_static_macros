use quote::{format_ident, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{ItemFn,FnArg};
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
      let tid = to_class_case(&format!("{}Template", f.sig.ident));
      let tid = format_ident!("{}", tid, span=f.sig.ident.span());
      let mut tfs = Vec::new();
      for i in f.sig.inputs.iter() {
         if let FnArg::Typed(p) = i {
            tfs.push("");
         }
      }

      quote_spanned!(syn::spanned::Spanned::span(f)=>
         struct #tid {
            xhtml: String,
         }
         impl #tid {
            pub fn new() -> #tid {
               #tid {
                  xhtml: std::default::Default::default(),
               }
            }
            pub fn set_xhtml(mut self, xhtml: String) -> #tid {
               self.xhtml = xhtml;
               self
            }
            pub fn build(&self) -> String {
               "".to_string()
            }
         }
         #f
      ).to_tokens(tokens);    
   }
}
