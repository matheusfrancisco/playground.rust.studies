use num::{BigUint, One};
use rayon::prelude::*;
use std::time::Instant;

fn factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one();
    } else {
        (1..=num)
            .map(BigUint::from)
            .reduce(|acc, x| acc * x)
            .unwrap()
    }
}

fn multi_fact(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one();
    } else {
        (1..=num)
            .into_par_iter()
            .map(BigUint::from)
            .reduce(|| BigUint::one(), |acc, x| acc * x)
    }
}

fn main() {
    let now = Instant::now();
    factorial(50000);
    println!("{:.2?}", now.elapsed());

    let now = Instant::now();
    multi_fact(50000);
    println!("{:.2?}", now.elapsed());
}
