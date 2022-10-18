


fn main() {
    let greater = return_greater(10, 5);
    println!("{}", greater);

    let mut original = String::from("original value");
    println!("outer scope original {}", original);

    {
        print_og(&original);
        change_og(&mut original);
        println!("ineer {}", original);
    }

}

fn print_og(original: &String) {
    println!("fn print_original: \t\"{}\"", original);
}

fn change_og(original: &mut String) {
    let next = original;
    *next = String::from("new value");
    println!("fn change {}", next)
}

fn return_greater(f: u8, s: u8) -> u8 {
    if f > s {
        f
    } else {
        s
    }
}

main()
