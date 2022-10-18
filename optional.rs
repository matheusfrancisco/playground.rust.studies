fn main() {

    let phrase: String = String::from("Duck Airlines");
    let letter = phrase.chars().nth(5);
    let letter2 = phrase.chars().nth(15);

    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No value")
    };

    let value2 = match letter2 {
        Some(character) => character.to_string(),
        None => String::from("No value")
    };

    println!("{}", value);
    println!("{}", value2);
    
}


main()
