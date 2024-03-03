mod shapes;
use crate::shapes::{area::Area, circle::Circle, rect::Rect};

fn main() {
    let rect = Rect::default();

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("{}", rect);
    println!("{}", rect.area());
    println!("{}", circ.area());
}
