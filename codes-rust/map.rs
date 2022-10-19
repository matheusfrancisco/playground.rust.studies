use std::collections::HashMap;

fn main() {
    let mut f = HashMap::new();

    f.insert("A", ("A", "1"));
    f.insert("a", ("A", "1"));
    f.insert("d", ("A", "1"));
    f.insert("A", ("A", "1"));

    let option = flights.get("A");
    let (time, destination) = option.unwrap();
    println!("{} {}", time, destination);


    if !f.contains_key("a") {
        f.insert("a", ("a", "2"));
    } else {
        println!("nnnn");

    }

    for ff in f.iter() {
        println("{}", ff);
    }

}


main()
