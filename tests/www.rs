use rdxl_static_macros::*;

pub mod static_files { 
   use rdxl_static_macros::*;

   #[dot_template]
   fn template1(xhtml: String) -> String {
      format!("{}", xhtml)
   }


   #[dot]
   fn file() -> String {
      dot_html!(
         template=::template1,
         "abcd"
      )
   }

   #[dot(suffix=".txt")]
   fn file2() -> String {
      "aaa".to_string()
   }
}

#[dot_template]
fn template1(title: String, xhtml: String) -> String {
   format!("{}{}", title, xhtml)
}

#[dot]
fn index() -> String {
   dot_html!(
      template=::template1,
      <p>Dot Hello</p>
   )
}
