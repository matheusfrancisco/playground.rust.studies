//Box<T>
//Rc<T>
//Ref<T>
//RefCell<T>
//Cell<T>
//Mutex<T>
//Arc<T>
//RwLock<T>
//Cow<T>
//Cell<T>

mod rc;
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;

use List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

fn main() {
    let b = Box::new(5); // b is a Box that points to the value 5
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //    The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation. These two traits will be even more important to the functionality provided by the other smart pointer types we’ll discuss in the rest of this chapter. Let’s explore these two traits in more detail.

    let x = 5;
    let y = &x;
    let k = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *k);

    let x = 3;
    let y = MyBox::new(x);

    assert_eq!(3, x);
    assert_eq!(3, *y);
    assert_eq!(3, *(y.deref()));
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
