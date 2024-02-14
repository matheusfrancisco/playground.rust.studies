fn main() {
    let mut nums: Vec<i32> = Vec::new();
    nums.push(1);
    nums.push(2);
    nums.push(4);

    let pop = nums.pop(); // Option<T>, return None or Some(T)
    println!("{:?}", pop);

    let two = nums[1]; //copy
    println!("{:?}", two);
    let one = nums.first();
    println!("{:?}", one);

    let mut flights: Vec<&str> = Vec::new();

    let vec_macro = vec![1, 2, 3, 4];

    flights.push("Add to the end 1");
    flights.push("Add to the end 2");
    flights.push("Add to the end 3");
    flights.push("Add to the end 4");

    let third = flights[2];
    println!("{}", third);

    let fourth = flights.get(3);

    match fourth {
        Some(flights) => {
            println!("{}", flights);
        }
        _ => {}
    }

    if let Some(flight_value) = flights.get(4) {
        println!("{}", flight_value);
    }

    for f in flights.iter() {
        println!("{}", f);
    }

    //remove index 1
    flights.remove(1);
    //.last
    // .first_mut and .last_mut, so will borrow mutable references

    // insert at index
    nums.insert(0, 10);
    // index 3
    nums.remove(2);

    nums.sort();
    nums.reverse();
}

// Some(4)
// 2
// Some(1)
