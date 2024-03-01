enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!(" red"),
        Color::Green => println!(" green"),
        Color::Blue => println!(" blue"),
        Color::Yellow => println!(" yellow"),
    }
}

fn main() {
    let file = std::fs::read_to_string("lines").unwrap();
    file.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));

    print_color(Color::Green);
    let foo = Color::Yellow;

    println!("{}", foo.is_green());
    println!("{}", foo.is_green_parts());
}
