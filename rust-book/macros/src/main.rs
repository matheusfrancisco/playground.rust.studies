use macros::vec;
mod declarative;

fn main() {
  let v1: Vec<i32> = Vec::new();
  let v2: Vec<&str> = vec!("a", "b", "c");
  println!("{:?}", v2);
}
