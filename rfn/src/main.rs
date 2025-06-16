type Transformation = fn(String) -> String;

fn main() {
    let input = "Hello, world!".to_string();
    let result = transform_string(input, to_uppercase);
    let result = transform_string(result, reverse);
    println!("Transformed string: {}", result);

    let to_upper = |input: String| input.to_uppercase();
    let rev = |input: String| input.chars().rev().collect::<String>();
    let result = transform_string("Hello, world!".to_string(), to_upper);
    let result = transform_string(result, rev);
    println!("Transformed string with closures: {}", result);
}

fn transform_string(input: String, transformation: Transformation) -> String {
    transformation(input)
}

fn to_uppercase(input: String) -> String {
    input.to_uppercase()
}

fn reverse(input: String) -> String {
    input.chars().rev().collect()
}
