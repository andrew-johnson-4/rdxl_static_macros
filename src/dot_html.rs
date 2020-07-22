use quote::{quote_spanned, ToTokens};
//use proc_macro2::{Span};
use syn::parse::{Parse, ParseStream, Result};
use rdxl_internals::xhtml::Xhtml;

pub struct DotHtmlInvocation {
   xhtml: Xhtml
}

impl Parse for DotHtmlInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let xhtml: Xhtml = input.parse()?;

        Ok(DotHtmlInvocation {
            xhtml: xhtml,
        })
    }
}

impl ToTokens for DotHtmlInvocation {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      let ref xhtml = self.xhtml;
      quote_spanned!(self.xhtml.span()=>
         #xhtml
      ).to_tokens(tokens);    
   }
}
