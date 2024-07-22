#[derive(Debug)]
enum Language {
    English,
    Spanish,
    Russian,
    Japonese,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Message2 {
    Hello { id: i32 },
}

fn main() {
    // at operator

    let msg = Message2::Hello { id: 5 };
    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello { id: 10..=17 } => println!("Found an id in another range"),
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
    // match guards
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
    let x = Some(5);
    let y = 10;
    match x {
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    //range
    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, .., last) => {
            println!("First: {}, Last: {}", first, last);
        }
    }
    //  ignoring
    let mut setting_value = Some(5);
    let new_settings_value = Some(10);

    match (setting_value, new_settings_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value")
        }
        _ => setting_value = new_settings_value,
    }
    println!("setting is {:?}", setting_value);
    // compicated
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // message
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change color: red {}, green {}, blue {}", r, g, b)
        }
    }
    // Language
    let language = Language::English;
    match language {
        Language::English => println!("Hello"),
        Language::Spanish => println!("Hola"),
        Language::Russian => println!("..."),
        lang => println!("Unsuported lang {:?}", lang),
    }

    let auth: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = auth {
        println!("Status : {}", status);
    } else if is_admin {
        println!("Admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Group id is greater than 30");
        } else {
            println!("Group id is less than 30");
        }
    } else {
        println!("No status");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let x = 5;
    // let PATTERN = EXPRESSION
    let (x, y, z) = (1, 2, 3);
    let point = (3, 5);
    print_coordinates(&point);

    //irrefutable PATTERN
    // let x = 5;
    //Refutable PATTERN
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

    // can only accept irrefutable patterns
    // function parameters are irrefutable
    // let statement can accept refutable patterns
    // if let, while let, for loop can accept refutable patterns
    //
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 5;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;

    match p {
        Point { x, y: 0 } => {
            println!("On the x axis at {}", x)
        }
        Point { x: 0, y } => {
            println!("On the y axis at {}", y)
        }
        Point { x, y } => println!("On neither axix ({}, {})", x, y),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
