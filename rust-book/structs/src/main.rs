struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from(""),
        sign_in_count: 1,
        active: true,
    };
    
    println!("{}", user1.username);
    
    

    println!("Hello, world!");
}
