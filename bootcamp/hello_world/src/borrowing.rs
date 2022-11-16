
/*
* Borrowing
* the act of creating a reference
* references are pointers with rules/restrictions
* references do not take ownership
*
* Why borrow?
* perfomance
* when ownership is not needed/desired
*
* Borrowing rules
* rules
* at any give time, you can have either one mutable referencce or any number of immutable references
* references must always be valid.
*
* problems these rules solve
* data races
* dangling references
*
*/


fn main() {

    let mut s1 = String::from("Rust");
    let r1 = &s1;
    print_string(r1);

    let r2 = &mut s1;
    add_to_string(r2);
    println!("s1 is {}", s1);
    let s2 = generate();

}

fn generate() -> String {
    String::from("Xico");
}

fn add_to_string(p: &mut String) {
    p.push_str(" is top!");
}

fn print_string(p1: &String) {
    println!("P1 {}", p1);
}


main()

