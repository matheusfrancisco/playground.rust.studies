use std::{cell::RefCell, rc::Rc};
// mutate data even when it is immutable

struct Flagger {
    is_true: Rc<RefCell<bool>>,
}

fn main() {
    let flag = Flagger {
        is_true: Rc::new(RefCell::new(true)),
    };
    //borrow return Ref<T> the smartpointer
    //borrow_mut return RefMut<T>

    //let reference = Rc::new(flag.is_true.clone());
    //println!("ref = {:?}", reference);

    let mut mut_ref = flag.is_true.borrow_mut();
    *mut_ref = false;
    println!("mut_ref = {:?}", mut_ref);
}
