struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let coord = Point { x: 1, y: 2 };
    println!("{}", coord.x());
    println!("{}", coord.y());

    let coord = Point { x: 1, y: '2' };

    println!("{}", coord.x());
    println!("{}", coord.y());
}
