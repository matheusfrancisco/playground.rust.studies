fn main() {
    let mut x = 5;
    // mut allow us to modify the value of the variable
    println!("The value of x is: {}", x);
    x = 3;
    println!("The value of x is: {}", x);

    let x: i8 = 10;
    //use unsigned integers when you know the value will never be negative
    println!("The value of x is: {}", x);
    let y: u8 = 10;
    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binay = 0b1111_1111;
    println!("Decimal: {}", decimal);
    println!("Hex: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}", binay);

    let byte = b'A';
    println!("Byte: {}", byte);

    println!("Max i8: {}", i8::MAX);
    println!("Max i16: {}", i16::MAX);
    println!("Max i32: {}", i32::MAX);
    println!("Max i64: {}", i64::MAX);

    println!("greatest common divisor: {}", gcd(20, 4));
    //boolean
    let t = true;
    let f: bool = false;
    println!("t: {}", t);

    //char
    let c = 'z';
    println!("c: {}", c);

    // operations + - * / %
}

// implement a function to calculate the greatest common divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if b < a {
            let temp = a;
            a = b;
            b = temp;
        }
        b = b % a;
    }
    a
}

//output of main
// The value of x is: 5
// The value of x is: 3
// The value of x is: 10
// Decimal: 255
// Hex: 255
// Octal: 255
// Binary: 255
// Byte: 65
// Max i8: 127
// Max i16: 32767
// Max i32: 2147483647
// Max i64: 9223372036854775807
// greatest common divisor: 4
// t: true
// c: z
