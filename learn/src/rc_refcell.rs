use std::{cell::RefCell, rc::Rc};

//https://doc.rust-lang.org/book/ch15-04-rc.html
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct Flagger {
    is_true: Rc<RefCell<bool>>,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let flag = Flagger {
        is_true: Rc::new(RefCell::new(true)),
    };
    //borrow return Ref<T>
    //borrow_mut return RefMut<T>

    let reference = Rc::new(flag.is_true.clone());
    println!("ref = {:?}", reference);

    let mut mut_ref = flag.is_true.borrow_mut();
    *mut_ref = false;
    println!("mut_ref = {:?}", mut_ref);
}
