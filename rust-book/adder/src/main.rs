#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn failed() {
        assert_eq!(1, 2, "1 is not equal to 2");
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err("2 + 3 != 4".to_string())
        }
    }
}
