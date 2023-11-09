struct MyString<'a> {
    text: &'a str,
}

fn main() {
    /*error
      let r;
      {
          let x = 5;
          r = &x;
      } // x is dropped
      println!("r: {}", r);
    */

    // &i32
    // &'a i32
    // &'a mut i32
    let x = example("hello");
    println!("x: {}", x);
    let str1 = String::from("abcd");
    let str2 = "xyz";
    let x = MyString { text: &str1 };
}

fn example<'a>(x: &'a str) -> &'a str {
    x
}
