struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
  fn myname(&self) -> &str {
      &self.username
  }
}

fn main() {
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from(""),
        sign_in_count: 1,
        active: true,
    };

    println!("{}", user1.username);

    let u = build_user(String::from("user1"), String::from("user1"));
    println!("Hello, world!");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let n = user1.myname();

    println!("{}", n);
    
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
