use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    for i in 11..13 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    //handle.join().unwrap();
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // passaging message throught thread
    // multiple producer single consumer
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        let val = String::from("Hi from the thread");
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("the3"),
            String::from("thread2"),
        ];
        let val = String::from("Hi from the thread");
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        tx2.send(val).unwrap();
    });
    for received in rx {
        println!("Got: {}", received);
    }
}
