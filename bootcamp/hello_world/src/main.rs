const MAX_PLAYERS: u8 = 10;
static CASINO_NAME: &str = "Rust Casino";
static mut CASINO_NAME2: &str = "Rust Casino";

fn main() {
    println!("Hello, world!");
    // Data types

    //arrays
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    //tuples
    let t1 = (1, 2, 3);

    let t2: (i32, i32, &str) = (1, 2, "5");
    // get a tuple value
    let s1 = t2.2;

    //destructuring an tuple
    let (i1, f1, s2) = t2;

    //empty tuple is when functions doesnt return values
    //it return an empty tuple call unit
    let unit = ();

    //type alias
    //type age = u8;
    //let a1: age = 57;

    // using constant variables;
    let b = 10;
    let c = CASINO_NAME;

    //
    // basic control flow

    let a = 10;
    if a > 8 {
        println!("bigget than");
    } else if a > 3 {
        println!("bigget than 3");
    } else {
        println!("smaller than 3");
    }

    // run forever;
    // loop {
    //     println!("loop forever");
    //     //this will break this loop tha outer will continue run
    //     loop {
    //         break;
    //     }
    // }

    // run forever;
    'outer: loop {
        println!("loop forever");
        loop {
            break 'outer;
        }
    }

    //loops can return values
    let x = loop {
        break 5;
    };


    let mut a = 0;

    while a < 5 {
        a = a + 1;
        a += 1;
    }


    let a = [1, 2, 3, 4 ,5];
    for element in a {
        println!("{}", element);
    }
}

fn my_fn(x: i32) -> i32 {
    println!("my_fn : {}", x);
    x
}
