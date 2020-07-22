use rdxl_static_macros::*;

#[dot_template]
fn template1(title: String, xhtml: String) -> String {
   format!("{}{}", title, xhtml)
}

#[test]
fn test_template1() {
   assert_eq!(
      template1_TEMPLATE::new()
               .set_xhtml("foobar")
               .build(),
      "foobar"
   );
}
