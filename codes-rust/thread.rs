
use std::thread;

fn main() {
    let out = 412;
    let join = thread::spawn(|| { out * 2});
    let result = join.join();

    match result {
        Ok(value) => {
            println!("{}", value);
        }
        Err(_) => {}
    }
}
