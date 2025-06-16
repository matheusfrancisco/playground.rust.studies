use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;

#[derive(Debug)]
struct User {
    id: String,
}

async fn lookup(id: &str, users: Arc<RwLock<Vec<User>>>) {
    let mut db = users.write().await;
    db.push(User { id: id.to_string() });
}

async fn call() {
    let users = Arc::new(RwLock::new(vec![]));
    let user_for_task = users.clone();

    let lookup_users = spawn(async move {
        lookup("bob", user_for_task.clone()).await;
        lookup("tom", user_for_task).await;
    });

    lookup_users.await.expect("failed to lookup users");
    let users_with_access = users.read().await;
    println!("Users: {:?}", users_with_access);
}

#[tokio::main]
async fn main() {
    let mut tasks = vec![];
    for id in 0..5 {
        let t = spawn(async move {
            println!("Async task {} started.", id);
            sleep(Duration::from_millis((5 - id) * 100)).await;
            println!("Async task {} done.", id);
            let result = id * id; // silly calculation...
            (id, result)
        });

        tasks.push(t);
    }

    println!("Launched {} tasks...", tasks.len());
    for task in tasks {
        let (id, result) = task.await.expect("task failed");
        println!("Task {} completed with result: {}", id, result);
    }
    println!("Ready!");

    call().await;
}
