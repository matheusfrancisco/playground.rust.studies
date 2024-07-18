mod ownership;

fn main() {
    let x = 5;
    //x = 6; x is immutable
    //to mut add mut
    let mut y = 5;
    y = 6;
    println!("The value of y is: {}", y);

    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("The value of SUBSCRIBER_COUNT is: {}", SUBSCRIBER_COUNT);

    // Integers
    // Length   Signed  Unsigned
    // 8-bit    i8      u8  = 1 byte -> max number is 255
    // 16-bit   i16     u16 = 2 bytes
    // 32-bit   i32     u32 = 4 bytes // Default
    // 64-bit   i64     u64 = 8 bytes
    // 128-bit  i128    u128 = 16 bytes
    // arch     isize   usize = depends on the computer architecture
    //
    // Floating point number
    // Rust also has two primitive types for floating-point numbers, which are numbers with
    // decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in
    // size, respectively. The default type is f64 because on modern CPUs it’s roughly
    // the same speed as f32 but is capable of more precision.
    // Boolean
    // Character
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    //tuple
    //
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    //arrays
    // In Rust, every element of an array must have the same type. Arrays in Rust have a fixed
    // length: once declared, they cannot grow or shrink in size.
    //
    let a = [1, 2, 3, 4, 5];
    let byte = [0; 5]; // [0, 0, 0, 0, 0]
                       //
    let a = [1, 2, 3, 4, 5];
    let index = 1;

    let element = a[index];

    println!("The value of element is: {}", element);
    another_function();
}

fn another_function() {
    println!("Another function.");
}

