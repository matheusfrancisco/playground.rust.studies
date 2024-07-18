//meory is managed through a system of ownership with a set of rules that the
//compiler checks at compile time. No run-time costs are incurred for any
//of the ownership features.

//1.Each value in Rust has a variable that’s called its owner.
//2.There can only be one owner at a time.
//3.When the owner goes out of scope, the value will be dropped.

pub fn helloy() {
    {
        let s = String::from("hello");
    } // s goes out of scope here
    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)
                 //
                 //takes_ownership1(s2); // s's value moves into the function...
                 // ... and so is no longer valid here.

    let x = 5; // x comes into scope.

    makes_copy(x); // x would move into the function,
    calculate_lenght(&s2);
    let mut s = String::from("hello world");
    // change(&mut s);

    let hello = &s[..5];
    let world = &s[6..];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
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
  //
fn calculate_lenght(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
