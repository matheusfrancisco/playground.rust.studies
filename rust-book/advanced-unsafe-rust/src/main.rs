use core::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *const i32;
    // ated *const i32 and *mut i32 raw pointers that both pointed to the same memory location, where num is stored.
    // If we instead tried to create an immutable and a mutable reference to num, the code would not have compiled
    // because Rust’s ownership rules don’t allow a mutable reference at the same time as any immutable references.
    // With raw pointers, we can create a mutable pointer and an immutable pointer to the same location and change
    // data through the mutable pointer, potentially creating a data race. Be careful!
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
