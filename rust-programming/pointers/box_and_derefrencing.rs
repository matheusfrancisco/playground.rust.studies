use std::{ops::Deref, rc::Rc};

//box will alocate the memory on the heap
//rather than the stack
fn main() {
    let t = (12, "eggs"); //create on the stack
    let b = Box::new(t); //create on the heap, but b was stored on the stack
    println!("{:?}", b);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // dealocate the memory-> returns the values on the memory address
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // same with a box
    assert_eq!(5, *y);
    println!("{:?}", y);
}
