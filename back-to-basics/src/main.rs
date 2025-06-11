use std::io::read_to_string;

use crate::utils::{greet, read_string, read_struct, write_struct, Person};

mod utils;
fn main() {
    let input = read_string();
    let person = Person::new(input);
    person.print();

    let list_of_names = vec!["Marcel", "Tom", "Dick", "Harry"];
    for first_name in list_of_names {
        greet(first_name);
    }
    for letter in "Hello".chars() {
        println!("{letter}");
    }
    write_struct(&person).expect("can not write to file");
    let person_from_file = read_struct().expect("can not read from file");
    person_from_file.print();
}
