fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing: Option<i8> = None;

    let y = 5;
    let x: Option<i8> = Some(5);
    println!("x is {:?}", x);

    let yx = some_to_number(x) + y;
    println!("yx is {}", yx); //evaluates yx is 10
                              //
    let phrase: String = String::from("Duck Airlines");
    let letter = phrase.chars().nth(5);
    let letter2 = phrase.chars().nth(15);

    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No value"),
    };

    let value2 = match letter2 {
        Some(character) => character.to_string(),
        None => String::from("No value"),
    };

    println!("{}", value);
    println!("{}", value2);
}

fn plus_one(x: Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
