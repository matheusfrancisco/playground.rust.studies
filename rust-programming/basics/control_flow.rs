fn main() {
    let one = 1;
    if one > 10 {
        println!("10");
    } else if one == 1 {
        println!("it is one");
    }

    let mut num = 0;
    'counter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;
        loop {
            println!("Decrease: {}", decrease);
            if decrease == 4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }
            decrease -= 1;
        }
        num += 1;
    }

    let mut num = 0;
    while num <= 10 {
        println!("Num: {}", num);
        num += 1;
    }

    let vec: Vec<i32> = (0..11).collect();
    for element in vec {
        println!("Element: {}", element);
    }

    let mut vec: Vec<i32> = (0..11).collect();
    for element in vec.iter().rev() {
        println!("Element: {}", element);
    }

    let available_aircraft = "Boeing";
    let minimum_crew = 7;
    let available_crew = 4;

    if (available_aircraft == "Boeing" || available_aircraft == "Airbus")
        && minimum_crew <= available_crew
    {
        println!("Ok");
    } else {
        println!("False");
    }

    if 1 == 2 {
        println!("oi");
    } else if 1 == 1 {
        println!("oi");
    }
}

// Count: 0
// Decrease: 5
// Decrease: 4
// Count: 1
// Decrease: 5
// Decrease: 4
// Count: 2
// Decrease: 5
// Num: 0
// Num: 1
// Num: 2
// Num: 3
// Num: 4
// Num: 5
// Num: 6
// Num: 7
// Num: 8
// Num: 9
// Num: 10
//
// Element: 0
// Element: 1
// Element: 2
// Element: 3
// Element: 4
// Element: 5
// Element: 6
// Element: 7
// Element: 8
// Element: 9
// Element: 10
//
// Element: 9
// Element: 8
// Element: 7
// Element: 6
// Element: 5
// Element: 4
// Element: 3
// Element: 2
// Element: 1
// Element: 0
