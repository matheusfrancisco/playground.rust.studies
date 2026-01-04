use std::{
    fs::File,
    io::{self, prelude::*},
};
const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main() {
    println!("Hello, world!");
}
"#;

pub fn fview_file(filename: String) -> std::io::Result<()> {
    let mut f = File::open(filename).expect("unable to open file");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!("");
        pos += BYTES_PER_LINE;
    }

    Ok(())
}

pub fn print() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    INPUT.read_to_end(&mut buffer)?;

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input);

        for byte in line {
            print!("{:02x} ", byte);
        }
        //print!("str: {}\n", String::from_utf8_lossy(line));
        println!();
        position_in_input += BYTES_PER_LINE
    }
    Ok(())
}
