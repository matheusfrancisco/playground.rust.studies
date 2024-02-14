fn main() {
    // you cannot have two mutable references in the same scope at the same time
    let mut s = String::from("hello");

    change_string(&mut s);
    println!("{}", s);
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

// hello, world
