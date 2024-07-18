pub fn main2() {
    println!("Hello, world!");
    another_function();
    my_fn(1, 2);
}

fn another_function() {
    println!("Another function.");
}

fn my_fn(x: i32, y: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

