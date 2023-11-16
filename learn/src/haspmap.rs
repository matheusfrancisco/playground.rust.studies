use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{team_name} : {score}");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    map.insert(1.to_string(), 10.to_string());
    println!("{:?}", map);

    let r = map.remove(&1.to_string());
    println!("{:?}", r);
    println!("{:?}", map);

    map.insert(1.to_string(), 10.to_string());
    println!("{:?}", map);

    let r = map.remove_entry(&1.to_string());
    println!("{:?}", r);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
