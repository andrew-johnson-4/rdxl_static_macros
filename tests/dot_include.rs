use rdxl_static_macros::dot_include;

#[test]
fn dot_include1() {
   let (fp,f) = dot_include!("post/entry1.html");
   assert_eq!(fp, "dot_include/entry1.html");
   assert_eq!(f(), "<p>Hello!</p>");
}
