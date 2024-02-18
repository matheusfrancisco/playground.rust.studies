use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!(" Guess the number!");

    let sec: i32 = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect(" Failed to read line");
        println!(" You guess: {}", guess);

        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Error");
                continue;
            }
        };

        match guess.cmp(&sec) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", " Too big!".red()),
            Ordering::Equal => {
                println!("{} ", "Win".green());
                break;
            }
            _ => println!("Error"),
        }
    }
}
