fn match() {
    match item {
        //match a single value
        1 => println!("one"),
        //match multiple values range inclusive
        10..=12 => println!("between ten and twelve"),
        // match | values on either side
        15 | 17 => println!("fifteen or seventeen"),
        _ => println!("something else"),
    }
}
