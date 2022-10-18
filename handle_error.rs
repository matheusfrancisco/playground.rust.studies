use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let filename = "customer.json";
    // let file_handle = File::open(filename);

    match File::open(filename) {
        Ok(file) => {
            println!("{:#?}", file);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(file) => {
                    println!("file created");
                }
                Err(error) => {
                    println!("{:#?}", error);
                }
            },
            _ => {
                println!("{:#?}", error);
            }
        },
    }
}
