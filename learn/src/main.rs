/* Ownership
* https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
* Notes:
*
* Because ownership is a new concept for many programmers, it does take some time to get
* used to. The good news is that the more experienced you become with Rust and the
* rules of the ownership system, the easier you’ll find it to naturally develop
* code that is safe and efficient. Keep at it! When you understand ownership
* you’ll have a solid foundation for understanding the features that make Rust unique.
* In this chapter, you’ll learn ownership by working through some examples that focus on
* a very common data structure: strings.
*
* Ownership rule:
* 1. Each value in Rust has a variable that’s called its owner.
* 2. There can only be one owner at a time.
* 3. When the owner goes out of scope, the value will be dropped.
*/
/*
*When a variable goes out of scope, Rust calls a special function for us. This function is called
drop, and it’s where the author of String can put the code to return the memory.
https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop
Rust calls drop automatically at the closing curly bracket.

If you’ve heard the terms shallow copy and deep copy while working with other languages,
the concept of copying the pointer, length, and capacity without copying the data probably
sounds like making a shallow copy. But because Rust also invalidates the first variable,
instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into
* */

fn main() {
    let mut s = String::from("hello");
    // it will change the value of s without take Ownership
    change_string(&mut s);
    println!("{}", s);
    // end
    //
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
