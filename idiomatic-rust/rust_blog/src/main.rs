use rust_blog::{Post, Role, User};

fn main() {
    let user = User::new("chico".to_owned(), Role::Creator).unwrap_or_default();
    let user2 = User::new("chicoo".to_owned(), Role::Creator).unwrap_or_default();
    println!("{:?}", user);
    println!("{:?}", user2);

    let post1 = Post::default();
    println!("{:?}", post1);
    let post2 = Post::new("Hello, world!".to_owned());
    println!("{:?}", post2);
}
