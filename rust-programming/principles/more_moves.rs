fn main() {
    let s = String::from("takes");
    takes_ownership(s);
    // error->    println!("{}", s);
    //     takes_ownership(s);
    //                     ^ value moved here
    //     println!("{}", s);
    //                    ^ value borrowed here after move
    //     let s = String::from("takes");
    //         ^ move occurs because `s` has type `String`, which does not implement the `Copy` trait
    // borrow of moved value: `s`
    // help: consider cloning the value if the performance cost is acceptable
    //
    let x = 1;
    make_copy(x);

    let str1 = give_ownership();
    println!("{}", str1);
    let str3 = take_and_give(str1);
    println!("{}", str3);

    if (true) {
        let str4 = str3;
    } else {
        let str5 = str3;
    }
    // if try to print println!("{}", str3);
    // help: if this is intentional, prefix it with an underscore
    // _str5
    //                    ^^^^ value moved here
    //         let str4 = str3;
    //                    ^^^^ value moved here
    //     println!("{}", str3);
    //                    ^^^^ value borrowed here after move
    //     let str3 = take_and_give(str1);
    //         ^^^^ move occurs because `str3` has type `String`, which does not implement the `Copy` trait
    // borrow of moved value: `str3`
    // help: consider cloning the value if the performance cost is acceptable
    // .clone()
    // help: consider cloning the value if the performance cost is acceptable
    // .clone()

    let mut s = String::from("chico");
    // loop needs to keep ownership of s
    loop {
        s = s + "o";
        println!("{}", s);
        if s.len() > 10 {
            break;
        }
    }
}

fn takes_ownership(s: String) {
    let strin = s;
    println!("Here: {}", strin);
}

fn make_copy(one: i32) {
    let val = one;
    println!("Val: {}", val);
}

fn give_ownership() -> String {
    "give".to_string()
}

fn take_and_give(str2: String) -> String {
    str2
}

//main()
// Here: takes
// Val: 1
// give
// give
// chicoo
// chicooo
// chicoooo
// chicooooo
// chicoooooo
// chicooooooo
