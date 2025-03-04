use std::str::FromStr;

use anyhow::Result;

enum Option2<T> {
    Some(T),
    None,
}

impl<T> Option2<T> {
    fn is_some(&self) -> bool {
        match *self {
            Option2::Some(_) => true,
            Option2::None => false,
        }
    }
}

fn error_me(throw: bool) -> Result<(), usize> {
    if throw {
        return Err(5);
    }
    Ok(())
}

fn get_input() -> &'static str {
    return "forward 5
down 5
forward 8
up 3
down 8
forward 2
";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_line(line: &str) -> Point {
    let (dir, amount) = line.split_once(" ").expect("must contains a whitespace");
    let amount = str::parse::<i32>(amount).expect("must be a integer");

    if dir == "forward" {
        return Point { x: amount, y: 0 };
    } else if dir == "up" {
        return Point { x: 0, y: amount };
    }
    return Point { x: 0, y: -amount };
}

fn get_input2() -> &'static str {
    return "0,9 -> 5,9
5,9 -> 5,2
5,2 -> 2,2
2,2 -> 2,7
0,9 -> 0,4
0,9 -> 2,9
3,4 -> 1,4
";
}

struct Line {
    p1: Point,
    p2: Point,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let result = s.split_once(",");
        if result.is_none() {
            return Err(anyhow::anyhow!("must contains a comma"));
        }
        let (x, y) = result.unwrap();
        let x: i32 = str::parse(x)?;
        let y: i32 = str::parse(y)?;

        Ok(Point { x, y })
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let result = s.split_once(" -> ");
        if result.is_none() {
            return Err(anyhow::anyhow!("must contains a ->"));
        }
        let (p1, p2) = result.unwrap();
        let p1: Point= str::parse(p1)?;
        let p2: Point= str::parse(p2)?;

        Ok(Line { p1, p2 })
    }
}

fn main() {
    //->Result<(), usize> {
    let some_number = Option2::Some(5);
    let no_number: Option2<i32> = Option2::None;

    println!("some_number.is_some() = {}", some_number.is_some());
    println!("no_number.is_some() = {}", no_number.is_some());

    //let val = match error_me(false) {
    //    Ok(_) => return Ok(()),
    //    Err(e) => return Err(e),
    //};

    //    if error_me(true).is_err() {
    //        println!("Error!");
    //    }
    //
    let result =
        get_input()
            .lines()
            .map(parse_line)
            .fold(Point { x: 0, y: 0 }, |mut acc, point| {
                acc.x += point.x;
                acc.y += point.y;
                return acc;
            });

    println!("result = {:?}", result);
}
