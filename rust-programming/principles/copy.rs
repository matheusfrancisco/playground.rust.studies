fn main() {
    let x = 1;
    let y = x;
    // there are some types that are copy by default
    // types stored in the stack are copy by default
    println!("{}", y);
    println!("{}", x);
}
