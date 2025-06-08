use crate::database::person::Person;
use crate::database::project::Project;

mod database;

fn main() {
    let p1 = Project {
        name: String::from("Project 1"),
    };

    let p = Person {
        name: String::from("Chico"),
        projects: vec![p1],
    };

    println!("{:?}", p);
}
