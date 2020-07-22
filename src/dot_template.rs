use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::ItemFn;

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
      quote_spanned!(syn::spanned::Spanned::span(f)=>
         #f
      ).to_tokens(tokens);    
   }
}
