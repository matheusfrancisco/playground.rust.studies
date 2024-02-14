use std::{ops::Deref, rc::Rc};

// this compile beacuse the size of the type is known at compile time and the values are stored on the stack
// using box if remove the box the size of the type is not known at compile time
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    /* When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
     * When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
     * When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type */

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
}
