fn main() {
    //let mut vec = vec![1, 2, 3];
    //print!("{:?}", vec);
    //let mut vec2 = &mut vec;
    //vec2.push(4);

    //println!("{:?}", vec2);

    //println!("{:p}", &vec2);
    //println!("{:p}", &vec);

    //let name = String::from("John");
    //let name2 = "Rust".to_string();
    //let name3 = name.replace("John", "te");

    //println!("{}", name);
    //println!("{}", name2);
    //println!("{}", name3);

    //let str1 = "hello";
    //let str2 = str1.to_string();
    //let str3 = &str2;
    //println!("{}", str3);
    //println!("{}", str2);
    //println!("{}", str1);
    // compare strings == , != (does not equal)
    println!("{}", gcd(20, 4));

    let mut num = 0;
    'counter: loop {
        println!("Outer: {}", num);
        let mut d = 5;
        loop {
            println!("D: {}", d);
            if d == 4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }
            d -= 1;
        }
        num += 1;
    }
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
