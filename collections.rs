//sequence collection
// lists that let you add, remove, update and search for values within the list at runtime
//
//
fn main() {
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
}

main()
