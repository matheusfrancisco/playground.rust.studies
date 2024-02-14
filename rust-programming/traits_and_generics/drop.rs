struct Course {
    headline: String,
    author: String,
}

impl Drop for Course {
    fn drop(&mut self) {
        println!("Dropping {}", self.headline);
    }
}

fn main() {
    let course = Course {
        headline: "Rust".to_string(),
        author: "Chico".to_string(),
    };
    //drop(course);
}
//still called drop
