fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("top is {}", top);
    }

    let xx = 15;
    match xx {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(10) => println!("ten"),
        Some(x) if x == y => println!("match y: {}", y),
        _ => println!("anything"),
    }
}

// top is 3
// top is 2
// top is 1
// something else
// anything
