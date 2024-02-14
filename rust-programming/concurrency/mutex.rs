use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
};

fn main() {
    let rc1 = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..8 {
        let counter = Arc::clone(&rc1);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            //let mut num2 = counter.lock().unwrap();
            //deadlock
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *rc1.lock().unwrap());

    let lock = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock);

    //poisoned
    let _ = std::thread::spawn(move || {
        let mut _num = lock2.lock().unwrap();
        panic!();
    })
    .join();

    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    *guard += 1;
    println!("Result: {}", *guard);
}
