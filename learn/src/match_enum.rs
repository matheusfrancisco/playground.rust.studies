
/*Enum and Match
*
**/

enum Pet {
    Dog,
    Cat,
    Fish,
}

impl Pet {
    fn what_am_i(&self) -> &'static str {
        match self {
            Pet::Dog => "dog",
            Pet::Cat => "cat",
            Pet::Fish => "flish",
            _ => "unknown",
        }
    }
}
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let d = Pet::Dog;
    println!("dog is {}", d.what_am_i());
    let f = Pet::Fish;
    println!("fish is {}", f.what_am_i());
    let c = Pet::Cat;
    println!("cat is {}", c.what_am_i());
    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.01")),
        address: String::from("localhost"),
    };
    println!("home is {}", home.address);
    println!("home kind is {:?}", home.kind);
    println!("home is {:?}", home);
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback is {}", loopback.address);

    //let some_number = Some(5)k
    //let some_string = Some("a string");
    //let nothing: Option<i8> = None;
    let y = 5;
    let x: Option<i8> = Some(5);

    let yx = some_to_number(x) + y;
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("yx is {}", yx);
    println!("six is {:?}", six);
    println!("none is {:?}", none);

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("top is {}", top);
    }

    let xx = 15;
    match xx {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(10) => println!("ten"),
        Some(x) if x == y => println!("match y: {}", y),
        _ => println!("anything"),
    }
}

fn some_to_number(x: Option<i8>) -> i8 {
    match x {
        Some(i) => i,
        None => 0,
    }
}

fn plus_one(x: Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
