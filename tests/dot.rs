use rdxl_static_macros::*;

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
