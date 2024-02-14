fn main() {
    print_phase("Print my argument");
    println!(
        "The greatest common divisor of 48 and 64 is {}",
        gcd(48, 64)
    );

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn print_phase(phrase: &str) {
    println!("{}", phrase);
    println!("This function is called print_phase");
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if b < a {
            let temp = a;
            a = b;
            b = temp;
        }
        b = b % a;
    }
    a
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main_greter() {
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

main_greter()
