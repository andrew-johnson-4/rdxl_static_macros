use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::{LitStr};

pub struct DotInclude {
   pub path: LitStr,
}

impl Parse for DotInclude {
    fn parse(input: ParseStream) -> Result<Self> {
        let path: LitStr = input.parse()?;

        Ok(DotInclude {
           path: path,
        })
    }
}

impl ToTokens for DotInclude {
   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      quote_spanned!(self.path.span()=>
         ("",||{ "".to_string() })
      ).to_tokens(tokens);    
   }
}
