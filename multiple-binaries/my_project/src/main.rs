//use database_lib::{person::Person, project::Project};
use database_lib::person as prs;
use database_lib::project as proj;



fn main() {
    let project = proj::Project {
        name: "Rust book".to_string(),
        description: "A book about Rust programming".to_string(),
    };
    let person = prs::Person {
        name: "Marcel".to_string(),
        projects: vec![project],
    };

    println!("{person:?}");
}
