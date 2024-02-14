use std::{ops::Deref, rc::Rc};

//Arc atomic reference counter use only to share pointer
//between threads

fn main() {
    let s = Rc::new(String::from("Pointer"));

    let s2 = s.clone();
    let s3 = s.clone();
    println!("{} {} {}", s.contains("Point"), s2, s3.contains("ter"));
}

/* When you have a type whose size can’t be known at compile time
* and you want to use a value of that type in a context that requires
* an exact size
*
 * When you have a large amount of data and you want to transfer
* ownership but ensure the data won’t be copied when you do so
*
 * When you want to own a value and you care only that it’s
* a type that implements a particular trait rather than being
* of a specific type */
