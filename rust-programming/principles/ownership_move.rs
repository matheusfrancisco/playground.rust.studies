/*
* Ownership rule:
* https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
* 1. Each value in Rust has a variable that’s called its owner.
* 2. There can only be one owner at a time.
* 3. When the owner goes out of scope, the value will be dropped.
*
*When a variable goes out of scope, Rust calls a special function for us. This function is called
drop, and it’s where the author of String can put the code to return the memory.
https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop
*/

fn main() {
    //push it on the rust stack
    let var = 1;
    let mut s = String::from("hello");
    s.push_str(", world");
    // allocate on the heap
    let x = vec!["tyler".to_string()];
    // this will error
    //    let y = x;
    //    println!("{:?}", x);
    //     let y = x;
    //         ^
    // unused variable: `y`
    // help: if this is intentional, prefix it with an underscore
    // _y
    //             ^ value moved here
    //     println!("{:?}", x);
    //                      ^ value borrowed here after move
    //     let x = vec!["tyler".to_string()];
    //         ^ move occurs because `x` has type `Vec<String>`, which does not implement the `Copy` trait
    // borrow of moved value: `x`
    // help: consider cloning the value if the performance cost is acceptable
    // .clone()
    // but this works
    let y = x;
    let z = y;
    println!("{:?}", z); // evaluates ["tyler"]
}

//var is dropped
// s is dropped
