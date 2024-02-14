// #![allow(unused)]

fn main() {
    let name = "Duck airlines";

    let write_message = |slogan: String| -> String {
        // println!("{} and {}", slogan, name);
        // println!("Hey this is the closure.");
        String::from(format!("{}", slogan))
    };

    let p = write_message(String::from("We hit the ground"));
    println!("{}", p);
}
