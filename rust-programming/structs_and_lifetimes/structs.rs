struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

struct Coordinates(i32, i32, i32);
//struct UnitStruct;
//1..4, .. => Range {start: 1, end: 4}
//

fn main() {
    let user = User {
        active: true,
        username: String::from("Chico"),
        sign_in_count: 0,
    };

    println!("active: {}", user.active);
    println!("username: {}", user.username);
    println!("sign_in_count: {}", user.sign_in_count);

    let user2 = build_user(String::from("Chico 2"));
    println!("active: {}", user2.active);
    println!("username: {}", user2.username);
    println!("sign_in_count: {}", user2.sign_in_count);

    let coord = Coordinates(1, 2, 3);
}

fn build_user(username: String) -> User {
    User {
        active: true,
        username,
        sign_in_count: 0,
    }
}

// active: true
// username: Chico
// sign_in_count: 0
// active: true
// username: Chico 2
// sign_in_count: 0
