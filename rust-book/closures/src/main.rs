use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculating: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculating: T) -> Cacher<T> {
        Cacher {
            calculating,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculating)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    //let expensive_result = simulated_expensive_calculation(intensity);
    let mut cached_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    //let example_closure = |x| x;
    //let s = example_closure(String::from("hello"));
    //let b = example_closure(1);

    if intensity < 25 {
        println!("today, do {}, pushups!", cached_result.value(intensity));
        println!("Next, do {}, situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
}

fn main() {
    let simulated_insensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_insensity, simulated_random_number)
}

//FnOnce, FnMut, Fn
//
//FnOnce: the closure captures the variables by value, so it can be called only FnOnce
//FnMut: the closure captures the variables by mutable reference, so it can be called FnMut or FnOnce
//Fn: the closure captures the variables by reference, so it can be called Fn, FnMut or FnOnce
