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
      let ref path = self.path;
      let filename = path.value();
      let filename = filename.rsplit("/").into_iter().next();

      quote_spanned!(self.path.span()=>
         (format!("{}/{}", std::module_path!().replace("::","/"), #filename), ||{
            include!(#path)
         })
      ).to_tokens(tokens);    
   }
}
