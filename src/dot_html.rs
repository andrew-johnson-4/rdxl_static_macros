use quote::{format_ident, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Path,Ident,Expr,parse_quote,Token};
use rdxl_internals::xhtml::Xhtml;
use inflector::cases::classcase::to_class_case;

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
      let mut path = self.template.clone();
      let ks = self.kwargs.iter().map(|(k,_)| format_ident!("set_{}",k)).collect::<Vec<Ident>>();
      let vs = self.kwargs.iter().map(|(_,v)| v.clone()).collect::<Vec<Expr>>();
      if let Some(i) = path.get_ident() {
         path = parse_quote!(crate::template::#i);
      } else {
         if path.leading_colon.is_some() {
            path.leading_colon = None;
         }
         if let Some(ref mut p) = path.segments.last_mut() {
            p.ident = format_ident!("{}", to_class_case(&format!("{}Template", p.ident)) );
         }
      }
      quote_spanned!(self.xhtml.span()=>
         #path::new()
             #(.#ks(#vs))*
             .set_xhtml({
                let mut stream = String::new();
                #xhtml
                stream
             })
             .build()
      ).to_tokens(tokens);    
   }
}
