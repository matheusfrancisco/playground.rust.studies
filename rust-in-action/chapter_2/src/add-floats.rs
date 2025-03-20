fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2); //<1>
    assert!(xyz.0 + xyz.1 == xyz.2); //<2>
}
// abc (f32)
//    0.1 + 0.2: 3e99999a
//          0.3: 3e99999a
// xyz (f64)
//    0.1 + 0.2: 3fd3333333333334
//          0.3: 3fd3333333333333
// thread '<unnamed>' panicked at src/lib.rs:53:5:
// assertion failed: xyz.0 + xyz.1 == xyz.2
