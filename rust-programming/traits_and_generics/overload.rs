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
            x: self.x + rhs.y,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let coord = Point { x: 1, y: 2 };
    let coord2 = Point { x: 1, y: 2 };
    let sum = coord + coord2;
    println!("{:?}", sum);
}
