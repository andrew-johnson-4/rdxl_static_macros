use quote::{quote_spanned, ToTokens};
//use proc_macro2::{Span};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Path,Ident,Expr,parse_quote,Token};
use rdxl_internals::xhtml::Xhtml;

pub struct DotHtmlInvocation {
   pub template: Path,
   pub kwargs: Vec<(Ident,Expr)>,
   pub xhtml: Xhtml
}

impl Parse for DotHtmlInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut template: Path = parse_quote!(crate::template::default);
        let mut kwargs = Vec::new();

        while input.peek(Ident) && input.peek2(Token![=]) {
           let id: Ident = input.parse()?;
           let _eq: Token![=] = input.parse()?;

           if id.to_string()=="template" {
              template = input.parse()?;
              let _comma: Token![,] = input.parse()?;
           } else {
              let expr: Expr = input.parse()?;
              let _comma: Token![,] = input.parse()?;
              kwargs.push((id, expr));
           }
        }

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
         Template1Template::new()
               .set_xhtml({
                  let mut stream = String::new();
                  #xhtml
                  stream
               })
               .build()
      ).to_tokens(tokens);    
   }
}
