use std::fs::{read_to_string, write};


pub struct Person {
    name: String,
}

impl Person {
    pub fn new(name: String) -> Self {
        Person { name }
    }
    pub fn print(&self) {
        println!("Hello {}", self.name);
    }
}

pub fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    let cleaned = input.trim().to_string();
    cleaned
}

pub fn greet(first_name: &str) {
    println!("{first_name}! I greet you.");
}

pub fn write_struct(person: &Person) -> std::io::Result<()> {
    let mut output = String::new();
    output.push_str(&person.name);
    output.push('\n');
    write("output.txt", output)
}

pub fn read_struct() -> Result<Person, std::io::Error> {
    let input = read_to_string("output.txt")?;
    let mut lines = input.split('\n');
    let name = lines.next().unwrap_or("Unknown").to_string();
    let person = Person::new(name);
    Ok(person)
}

