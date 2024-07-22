//pub trait Iterator {
//    type Item;
//    fn nex(&mut self) -> Option<Self::Item>;
//
//    // methods with default implementations elided
//}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![1, 2, 3, 4, 5];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
}

#[cfg(test)]
mod test {
    use crate::{Counter, Shoe};

    #[test]
    fn test_iter_next() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 12,
                style: String::from("sandal"),
            },
            Shoe {
                size: 11,
                style: String::from("boot"),
            },
        ];

        let in_my_size = super::shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![Shoe {
                size: 10,
                style: String::from("sneaker")
            }]
        );
    }

    #[test]
    fn counter_next() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iter_trait_methods_counter_next() {
        let addup: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(18, addup);
    }
}
