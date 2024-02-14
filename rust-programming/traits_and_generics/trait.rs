// traits

struct Course {
    headline: String,
    author: String,
}

trait Overview {
    fn overview(&self) -> String {
        String::from("No overview available")
    }
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

//it will use the default implementation
impl Overview for AnotherCourse {}

fn call_overview<T: Overview>(item: T) {
    println!("{}", item.overview());
}

fn call_overview2(item: &impl Overview) {
    println!("{}", item.overview());
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
fn main() {
    let course = Course {
        headline: "Rust".to_string(),
        author: "Chico".to_string(),
    };

    let course2 = AnotherCourse {
        headline: "Rust".to_string(),
        author: "Chico".to_string(),
    };

    println!("{}", course.overview());
    println!("{}", course2.overview());

    call_overview(course);
    call_overview2(&course2);
}
