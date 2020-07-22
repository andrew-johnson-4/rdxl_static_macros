use rdxl_static_macros::*;

#[dot_template]
fn template1(title: String, xhtml: String) -> String {
   format!("{}{}", title, xhtml)
}

#[test]
fn test_template1() {
   assert_eq!(
      Template1Template::new()
               .set_xhtml("foobar".to_string())
               .build(),
      "foobar"
   );
}

#[test]
fn test_template2() {
   assert_eq!(
      dot_html!(
        template=::template1,
        title="abc".to_string(),
        <p>A B C</p>
      ),
      "abc<p>A B C</p>"
   );
}
