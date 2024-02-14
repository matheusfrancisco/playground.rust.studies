fn main() {
    //string is similar to a vector
    // https://doc.rust-lang.org/std/string/struct.String.html
    //
    let name = String::from("Rust");
    let course = "Rust".to_string();
    println!("{} {}", name, course);

    let new_name = name.replace("Rust", "Programming");
    println!("{} {}", course, new_name);

    //&str string slice or str
    //going to reference and borrow the text from the vector
    //you cannot modify an string slice

    let stri1 = "hello";
    println!("{}", stri1);
    //    println!("{}", stri1.bogus());
    // eval (current-form): { //string is similar to a vector let name = Str...
    //     println!("{}", stri1.bogus());
    //                          ^^^^^ method not found in `&str`
    // no method named `bogus` found for reference `&str` in the current scope
    // when to use string or &str?
    // &str more to function arguments, string slice does not allocate memory
    // on the heap

    let str2 = stri1.to_string();
    let str3 = str2.as_str();
    let str4 = &str2;
    println!("{} {} {}", str3, str2, str4);

    // compare string == != (does not equal)
    println!("{}", "hello" == "hello");
}

// Rust Rust
// Rust Programming
// hello
// hello hello hello
// true
