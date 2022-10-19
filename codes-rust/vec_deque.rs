use std::collections::{VecDeque};

fn main() {
    let mut flights:VecDeque<&str> = VecDeque::new();

    flights.push_fron("Front");
    flights.push_back("Back");
    flights.push_fron("Front");
    flights.push_back("Front");

    let size = flights.len();

    let exists = flights.contains(&"Front");

    //iter()
    // iterates over collection and cannot change those values
    //iter.mut()
    //iterates over a collection and allows changing of those values
    
}
