use proc_macro2::TokenStream;
use quote::{quote,format_ident};

use syn::parse::{Parse, ParseStream, Result, Error};
use syn::{Token,LitStr,Ident};

pub struct Cfg {
   pub suffix: String,
}
impl Parse for Cfg {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut suffix = ".html".to_string();

        if input.peek(Ident) {
           let cfg_key: Ident = input.parse()?;
           if cfg_key.to_string()=="suffix" {
              let _eq: Token![=] = input.parse()?;
              let lit: LitStr = input.parse()?;
              suffix = lit.value();
           } else {
              let msg = format!("Unexpected dot configuration key: {}", cfg_key);
              let r = Error::new(cfg_key.span(), msg);
              return Err(r)
           }
        } else if !input.is_empty() {
           let msg = format!("Dot configuration key must be ident");
           let r = Error::new(input.span(), msg);
           return Err(r)
        }

        Ok(Cfg {
           suffix: suffix,
        })
    }
}

pub fn quote_itemfn(cfg: Cfg, f: syn::ItemFn) -> TokenStream {
   let fi = f.sig.ident.clone();
   let fctor = format_ident!("{}_ctor", fi ,span=fi.span());
   let fi_s = syn::LitStr::new(&fi.to_string(), fi.span());
   let dot_suffix = syn::LitStr::new(&cfg.suffix, fi.span());
   quote! {
      #f

      #[ctor::ctor]
      fn #fctor() {
         let fname = format!("{}/{}{}", std::module_path!().replace("::","/"), #fi_s, #dot_suffix);
         let dat = #fi();
         let pr = fname.rfind("/").expect("dot_to_file expected a / in directory path");
         let pd = fname[..pr].to_string();
         let p = std::path::Path::new(&fname);
         std::fs::create_dir_all(pd).expect("dot_to_file could not create directory");
         let mut f = std::fs::File::create(p).expect("dot_to_file could not create file");
         std::io::Write::write_all(&mut f, dat.as_bytes()).expect("dot_to_file could not write bytes to file");
      }
   }
}
