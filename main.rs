

fn main() {
    // let location: [f32;2] = [0.0, 0.0];
    let location: (&str, f64, f64) = ("KCLE", 41.10940669, -81.8546911);
    // destructuring 
    let (name, latitude, longitude) = location;
    println!("Localtion name: {}, latitude: {}, longitude: {}", location.0, location.1, location.2);
    let person_slice = "Matheus";
    let person_string = person_slice.to_string();

    let p_string = String::from("Matheus");
    let p_slice = &p_string;
    let p_slice2 = p_string.as_str();

    //concat strings
    let duck = "Duck";
    let airlines = "Air";
    let name_airline = format!("{} {}", duck, airlines);
    
    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "every time";
    println!("slogan: {}", slogan);

    // Integers
    // Booleans
    // Characters
    // Strings
    let float_thirty_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;
    //let cast_unsigned_eight = unsigned_eight as f32;


    let result = float_thirty_two / cast_unsigned_eight;
    println!("{}", result);
}

