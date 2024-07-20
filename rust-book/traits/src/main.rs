use std::fmt::{Debug, Display};

pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
    fn author_summary(&self) -> String {
        String::from("Jao")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//impl Summarizable for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

//fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
  1
}

fn main() {}
