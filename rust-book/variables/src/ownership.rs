//memory is managed through a system of ownership with a set of rules that the
//compiler checks at compile time. No run-time costs are incurred for any
//of the ownership features.

//1.Each value in Rust has a variable that’s called its owner.
//2.There can only be one owner at a time.
//3.When the owner goes out of scope, the value will be dropped.

fn main() {
    {
        let s = String::from("hello");
    } // s goes out of scope here
    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)
                 //
    takes_ownership1(s); // s's value moves into the function...
                         // ... and so is no longer valid here.

    let x = 5; // x comes into scope.

    makes_copy(x); // x would move into the function,
}

fn takes_ownership1(some_string: String) {
    // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
