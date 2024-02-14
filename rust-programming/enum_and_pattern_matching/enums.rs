enum NavigationAids {
    NDB,
    VOR,
    VORDME,
}

enum Pet {
    Dog,
    Cat,
    Fish,
}

impl Pet {
    fn what_am_i(&self) -> &'static str {
        match self {
            Pet::Dog => "I am dog",
            Pet::Cat => "I am cat",
            Pet::Fish => "I am fish",
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

    println!("NDB:\t{}", NavigationAids::NDB as u8);
    println!("VOR:\t{}", NavigationAids::VOR as u8);
    println!("VORDME:\t{}", NavigationAids::VORDME as u8);

    let d = Pet::Dog;
    println!("{}", d.what_am_i());
    let f = Pet::Fish;
    println!("{}", f.what_am_i());
    let c = Pet::Cat;
    println!("{}", c.what_am_i());

    //compounding enum and struct
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

}

 main()

// NDB:	0
// VOR:	1
// VORDME:	2
// I am dog
// I am fish
// I am cat
