use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number2 {
    value: i32,
}

impl Into<Number2> for i32 {
    fn into(self) -> Number2 {
        Number2 { value: self }
    }
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
