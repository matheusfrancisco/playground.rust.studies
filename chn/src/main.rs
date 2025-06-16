use tokio::{join, spawn, sync::mpsc};

#[derive(Debug)]
struct User {
    id: String,
}

async fn lookup_user(id: &str, mut user_chan: mpsc::Sender<User>) {
    println!("sending user: {id}");
    user_chan
        .send(User { id: id.to_string() })
        .await
        .expect("can not send user on channel");
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    let mut users = vec![];

    spawn(async move {
        join! {
            lookup_user("bob", tx.clone()),
            lookup_user("tom", tx),
        }
    });
    while let Some(user) = rx.recv().await {
        println!("received: {user:?}");
        users.push(user);
    }

    println!("Users: {users:?}")
}
