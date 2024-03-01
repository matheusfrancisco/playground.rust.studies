struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello Fem".to_string()));
}

fn main() {
    let mut items = vec![];
    append(&mut items);

    //let mut items: Vec<usize> = vec![];
    //append(&mut items);
    //
    let foo = Item::MyCustom(Custom {
        age: 100,
        name: "Jose".to_string(),
    });

    match &foo {
        Item::MyCustom(custom) if custom.name == "Chico" => println!("Hi, Chico"),
        Item::MyCustom(custom) if custom.age > 33 => println!("was the best"),
        Item::MyCustom(custom) if custom.age < 33 => println!("was not the best"),
        _ => {}
    }
}
