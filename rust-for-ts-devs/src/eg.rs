//struct Foo {
//  propertis...
//  pub propertis...
//}
//
//impl Foo {
//  // these are both static methods
//  fn this()
//  pub fn this() // public everyone can use
//
//  fn this(&self)
//  fn this(&mut self)
//
//  pub fn this(self)
//
//}
//
//Something people think is complicated
//String
//* well string is a heap allocated
//* can be mutable
//
//&str
//* this points to a sequence of utf-8 char
//* its immutable
//* its analogous to &[u8]
//
//
//
//
//fn a_function_that_can_error() -> Result<(), Err> {
//    Ok()
//}
//
//if let Ok(value) = a_function_that_can_error() {
//
//}
//match a_function_that_can_error() {
//    Ok(value) => println!("oh yeah {}", value),
//    Err(e) => eprintln!(" oh err {}", e),
//}
//
//_ = a_function_that_can_error();
//
//a_function_that_can_error().and_then().and_then()
//thiserror
//anyhow
