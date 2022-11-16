/*
 * Ownership
 * A strategy for managing memory (and other resources)
 * through a set a rules checked at compile time.
 * Rules.
 *  Each value in Rust has a variable that is called its owner
 *  There can only be one owner at time
 *  When the owner goes out of scope, the value will be dropped
*
*
*  Problems ownsership solves
*  memory resoure leaks
*  double free
*  use after free
*/

fn main() {

    // {
        // let s1 = String::from("Rust");
    // } // s1 dropped

    let s1 = String::from("Rust");
    print_string(s1.clone()); // this works
    let s2 = s1.clone();
    let s3 = generate_string();
    let s4 = add_to_string(s2);

    println!("S1 is : {s1}");
    // println!("S2 is : {s2}");
    println!("S3 is : {s3}");
    println!("S4 is : {s4}");

    // clone by default
    let x = 10;
    let y = x;
    print_integer(x);
    println!("x is : {x}");
    println!("y is : {y}");

} // s1 is dropped

fn print_integer(i: i32) {
    println!("i is {}", i);
}

fn add_to_string(mut p: String) -> String {
    p.push_str(" is top!");
    p
}

fn generate_string() -> String {
    String::from("Xico")
}

fn print_string(p1: String) {
    println!("P1 {}", p1);
}


main();

