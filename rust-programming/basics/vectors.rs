fn main() {
    //A contiguous growable array type, written as Vec<T>, short for ‘vector’.
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("After Push {:?}", nums);
    nums.pop();
    println!("After Pop {:?}", nums);

    let mut vec = Vec::new();
    vec.push("Test");
    vec.push("String");
    // https://doc.rust-lang.org/std/vec/struct.Vec.html

    let mut vect = Vec::<i32>::with_capacity(2);
    println!("Capacity: {}", vect.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    nums.extend([10, 20, 40]);
    println!("After Extends {:?}", nums);

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        println!("{top}");
    }

    //This is highly unsafe, due to the number of invariants that aren’t checked:
    //Creates a Vec<T> directly from a pointer, a capacity, and a length.
    //https://doc.rust-lang.org/std/vec/struct.Vec.html#examples-3
    //pub const fn new_in(alloc: A) -> Vec<T, A>
    //🔬This is a nightly-only experimental API. (allocator_api #32838)
    //
    // reserves capacity for at lest additional more elements to be inserted
    // in the given Vec<T>
    let mut vec_x = vec![1];
    vec_x.reserve(10);
    assert!(vec_x.capacity() >= 11);
    println!("{:?}", vec_x);

    //append
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);
}

// After Push [1, 2, 3, 4]
// After Pop [1, 2, 3]
// Capacity: 2
// [0, 1, 2, 3, 4]
// After Extends [1, 2, 3, 10, 20, 40]
// While
// 3
// 2
// 1
//
