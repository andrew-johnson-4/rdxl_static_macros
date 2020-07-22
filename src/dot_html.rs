use quote::{quote_spanned, ToTokens};
//use proc_macro2::{Span};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Path,Ident,Expr,parse_quote};
use rdxl_internals::xhtml::Xhtml;

pub struct DotHtmlInvocation {
   template: Path,
   kwargs: Vec<(Ident,Expr)>,
   xhtml: Xhtml
}

impl Parse for DotHtmlInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let template: Path = parse_quote!(crate::template::default);
        let kwargs = Vec::new();
        let xhtml: Xhtml = input.parse()?;

        Ok(DotHtmlInvocation {
            template: template,
            kwargs: kwargs,
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
