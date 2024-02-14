// terminate the current thread
fn main() {
    //panic!("panicked here!")
    panic_vec();
}

fn panic_vec() {
    let vector = vec![1, 2, 3, 4, 5];
    println!("{}", vector[0]);
    println!("{}", vector[10]);
}
