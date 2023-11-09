
/*
* the slice type
*
*Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
*A slice is a kind of reference, so it does not have ownership.

*/


fn main () {
  let s = "helloworlda".to_string();
  let len = first_word(&s);
  println!("first word is {}", len);

  let s = "hello worlda".to_string();
  let len = first_word(&s);
  println!("first word is {}", len);

  let s = "helloworldx a".to_string();
  let len = first_word2(&s);
  println!("first word is {}", len);

  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[6..11];
  println!("{}", hello);
  println!("{}", world);

  //let mut s = String::from("hello world");
  //let len = first_word2(&s);
  //s.clear(); // error
  //println!("the first word is: {}", len);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
  println!("bytes is {:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
