fn main() {
    let x = vec!["tyler".to_string()];
    let y = x.clone();
    let z = y.clone();
    println!("{:?}", z); // evaluates ["tyler"]
    println!("{:?}", y); // evaluates ["tyler"]
    println!("{:?}", x); // evaluates ["tyler"]
}
