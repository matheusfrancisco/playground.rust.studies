
use std::time::{Duration, Instant}; //<1>
fn main() {
    //for item in container this access the container and iterate over each item in the container

    for item in container {} // this get the ownership of the container
    for item in IntoIterator::into_iter(container) {} // this get the ownership of the container
                                                      // after the loop the container is no longer available
    for item in &container {} // this borrows the container
    for item in collection.iter() {}
    // after the loop the container is still available
    for item in &mut container {} // this mutably borrows the container
    for item in collection.iter_mut() {}

    // there is anonymous loop
    for _ in 0..10 {} // this is a loop that runs 10 times

    //avoiding managing an index variable
    let collection = vec![1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        println!("{}", collection[i]);
    }

    while condition {
        // code
    }

    //infinite loop
    loop {
        // code
    }

    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();
    count += 1;
    while (Instant::now() - start) < time_limit {
        //<4>
        count += 1;
    }
    println!("{}", count);
}
