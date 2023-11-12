//trait Clone: Sized {
//    fn clone(&self) -> Self;
//    fn clone_from(&mut self, source: &Self) {
//        *self = source.clone();
//    }
//}

use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let coord = Point { x: 5.0, y: 10.0 };
    let coord2 = Point { x: 1.0, y: 2.0 };
    let sum = coord + coord2;
    println!("{:?}", sum);
}
