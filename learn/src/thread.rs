use std::thread;
use std::time::Duration;

fn main() {
    //let handle = thread::spawn(|| {
    //    for i in 1..10 {
    //        println!("hi number {} from the spawned thread!", i);
    //        thread::sleep(Duration::from_millis(1));
    //    }
    //});

    //for i in 1..5 {
    //    println!("hi number {} from the main thread!", i);
    //}

    let v = vec![1, 2, 3];
    //let handle = thread::spawn(move || {
    //    println!("{:?}", v);
    //});
    let mut t_handles = Vec::new();
    for e in v {
        t_handles.push(thread::spawn(move || {
            println!("thread {:?}", e);
        }));
    }
    println!("Main thread");
    for handle in t_handles {
        handle.join().unwrap();
    }
}
