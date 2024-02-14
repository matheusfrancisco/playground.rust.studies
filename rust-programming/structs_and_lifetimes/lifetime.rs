fn main() {
    /*error
      let r;
      {
          let x = 5;
          r = &x;
      } // x is dropped
      println!("r: {}", r);
    and we try to reference it
    */

    // &i32 reference
    // &'a i32 reference with lifetiem
    // &'a mut i32 reference with lifetime mutable

    let x = example("hello");

    println!("x: {}", x);
    let str1 = String::from("abcd");
}

//fn example<'a>(x: &'a str) -> &'a str {
//    x
//}

fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
    y
}
