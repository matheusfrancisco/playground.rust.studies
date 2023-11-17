use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replacement: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!(
        "{} - replace string with new string",
        "Find and Replace".green()
    );
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <INPUT FILE>");
}

fn replace(target: &str, rep: &str, data: &str) -> Result<String, regex::Error> {
    let re = Regex::new(target)?;
    Ok(re.replace_all(data, rep).to_string())
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 4 {
        print_help();
        eprintln!(
            "{} wrong number of arguments give. Expected 4 got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    Arguments {
        pattern: args[0].clone(),
        replacement: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}

fn read_and_write(args: &Arguments) {
    let data = match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} failed to read from file {}: {:?}",
                "Error".red().bold(),
                args.input_file,
                e
            );
            std::process::exit(1);
        }
    };

    let replace_data = match replace(&args.pattern, &args.replacement, &data) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{} failed to replace pattern {}: {:?}",
                "Error".red().bold(),
                args.pattern,
                e
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output_file, &replace_data) {
        Ok(_) => println!(
            "{} successfully wrote to file {}",
            "Success".green().bold(),
            args.output_file
        ),
        Err(e) => {
            eprintln!(
                "{} failed to write to file {}: {:?}",
                "Error".red().bold(),
                args.output_file,
                e
            );
            std::process::exit(1);
        }
    }
}

pub fn run() {
    let args = parse_args();
    read_and_write(&args);
}
