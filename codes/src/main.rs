use crate::serde_eg::convert_struct;
mod fview;
mod serde_eg;
mod file;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    convert_struct();
    println!("\n");
    let _ = fview::print();
    println!("\n");
    // println!("root: {}", std::env::current_dir().unwrap().display());
    let _ = fview::fview_file("./src/main.rs".to_string());
}
