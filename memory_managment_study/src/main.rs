use std::ops::{Deref, DerefMut};

struct MySmartPtr<T> {
    value: T,
}

impl<T> MySmartPtr<T> {
    fn new(value: T) -> Self {
        MySmartPtr { value }
    }
}

impl<T> Deref for MySmartPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for MySmartPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let s = MySmartPtr::new(Box::new("hello".to_owned()));
    print(&s);
}

fn print(s: &str) {
    println!("{}", s);
}
