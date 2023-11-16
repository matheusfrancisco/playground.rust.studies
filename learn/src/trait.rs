
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

trait Overview {
    fn overview(&self) -> String;
}

struct Course {
    headline: String,
    author: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

fn call_overview<T: Overview>(item: T) {
    println!("{}", item.overview());
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let coord = Point { x: 1, y: 2 };
    println!("{}", coord.x());
    println!("{}", coord.y());
    let course = Course {
        headline: "Rust".to_string(),
        author: "Chico".to_string(),
    };
    println!("{}", course.overview());
    call_overview(course);
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

// allows multiple types item 1 and item 2
// fn overview(item: &impl Overview, item2: &impl Overview)

//has to be the same type
//fn overview<T: Overview>(item: &T, item2: &T)

//allows multiple types
//fn overview<T: Overview, U: Overview>(item: &T, item2: &U)

//
//fn overview(item: &impl Overview + AnotherTrait)
//fn overview<T: Overview + AnotherTrait>(item: &T)
