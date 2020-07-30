use proc_macro2::TokenStream;
use quote::{quote,format_ident};

pub fn quote_itemfn(f: syn::ItemFn) -> TokenStream {
   let fi = f.sig.ident.clone();
   let fctor = format_ident!("{}_ctor", fi ,span=fi.span());
   let fi_s = syn::LitStr::new(&fi.to_string(), fi.span());
   quote! {
      #f
      #[ctor::ctor]
      fn #fctor() {
         let fname = format!("{}/{}.html", std::module_path!().replace("::","/"), #fi_s);
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
