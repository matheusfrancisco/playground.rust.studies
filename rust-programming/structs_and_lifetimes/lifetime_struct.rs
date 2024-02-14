https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html?highlight=lifetime#lifetime-elision
struct MyString<'a> {
    text: &'a str,
}

fn main() {
    let str1 = String::from("abcd");
    let x = MyString {
        text: str1.as_str(),
    };
    println!("{}", x.text);
}
