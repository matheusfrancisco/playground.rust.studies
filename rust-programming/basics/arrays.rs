fn main() {
    let array = [1, 2, 3];
    println!("Array: {}", array[0]);
    println!("Array: {:?}", array);

    let mut array2: [i32; 3] = [5, 6, 7];
    println!("Array: {}", array2[0]);
    array2[0] = 10;
    println!("Array: {}", array2[0]);
    //     array[3] = 11;
    //     ^^^^^^^^^^^^^ cannot assign
    // cannot assign to `array[_]`, as `array` is not declared as mutable
    // help: consider changing this to be mutable
    // mut
}

// Array: 1
// Array: [1, 2, 3]
// Array: 5
// Array: 10
