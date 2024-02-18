fn main() {
    let twenty = 20; //<1>
    let twenty_one: i32 = 21; //<2>
    let twenty_two = 22i32; //<3>

    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000; //<4>
    println!("{}", one_million.pow(2)); //<5>

    let forty_twos = [
        //<6>
        42.0,     //<7>
        42f32,    //<8>
        42.0_f32, //<9>
    ];

    println!("{:02}", forty_twos[0]); //<10>
    let three = 0b11; //<1>
    let thirty = 0o36; //<2>
    let three_hundred = 0x12C; //<3>

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2:  {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8:  {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

    // assert!(0.1 + 0.2 == 0.3);
    //
    // for item in container {}
    //after this loop the container will be dropped if you want
    //to keep the container you can use the following syntax
    // for item in &container {}
    //
    // if you need modify each item during the loop, you can use a mutable reference
    // for item in &mut container {}
    //
}
//Number Type Bit pattern in memory
//20     u32  00000000000000000000000000010100
//20     i8   00010100
//20     f32  01000001101000000000000000000000
//
//Shorthand                  Equivalent to                                      Access
//for item in collection      for item in IntoIterator::into_iter(collection)   Ownership
//for item in &collection     for item in collection.iter()                     Read-only
//for item in &mut collection for item in collection.iter_mut()                 Read-write
//
//
//for _ in 0..10 {}
//for _ in 0..=10 {}
