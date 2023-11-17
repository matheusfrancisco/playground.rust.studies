use text_colorizer::*;
use std::env;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
  pattern: String,
  replacement: String,
  input_file: String,
  output_file: String,
}

fn print_help() {
  eprintln!("{} - replace string with new string", "Find and Replace".green());
  eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <INPUT FILE>");
}

pub fn run() {
  let args: Vec<String> = env::args().skip(1).collect();
  if args.len() < 4 {
    print_help();
    eprintln!("{} wrong number of arguments give. Expected 4 got {}", "Error".red().bold(), args.len());
    std::process::exit(1);
  }
}

