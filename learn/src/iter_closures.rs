use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct City {
    city: String,
    population: u64,
}

fn sort_pop(city: &mut Vec<City>) {
    city.sort_by_key(pop_helper)
}

fn pop_helper(pop: &City) -> u64 {
    pop.population
}

fn sort_pop_closure(pop: &mut Vec<City>) {
    pop.sort_by_key(|p| p.population)
}

//FnOnce
//impl<T> Option<T> {
//    pub fn unwrap_or_else<F>(self, f: F) -> T
//    where
//        F: FnOnce() -> T,
//    {
//        match self {
//            Some(x) => x,
//            None => f(),
//        }
//    }
//}

#[derive(Debug)]
struct Item {
    name: String,
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Iterator for Range {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

fn check_inventory(items: Vec<Item>, product: String) -> Vec<Item> {
    items
        .into_iter()
        .filter(|item| item.name == product)
        .collect()
}

fn main() {
    let cities = HashMap::from([("Mercury", 4), ("Venus", 7), ("Earth", 10), ("Mars", 15)]);
    let mut cities_vec = Vec::<City>::new();
    cities.iter().for_each(|(city, pop)| {
        cities_vec.push(City {
            city: city.to_string(),
            population: *pop as u64,
        })
    });

    println!("Before sort: {:?}", cities_vec);
    //sort_pop(&mut cities_vec);
    sort_pop_closure(&mut cities_vec);
    println!("Before sort: {:?}", cities_vec);

    let add = |x: i32| -> i32 { x + 1 };
    println!("add(1) = {}", add(1));

    let add_v2 = |x| x + 1;
    println!("add_v2(1) = {}", add_v2(1));

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("s = {}", s);

    /*FnOnce applies to closures that can be called once. All closures implement at
     *least this trait, because all closures can be called. A closure that moves
     *captured values out of its body will only implement FnOnce and none of the other
     *Fn traits, because it can only be called once.*/

    /*FnMut applies to closures that don’t move captured values out of their body
     * , but that might mutate the captured values. These closures can be called more than once.
     */

    /*Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values,
     * as well as closures that capture nothing from their environment. These
     * closures can be called more than once without mutating their environment, which is important in cases
     * such as calling a closure multiple times concurrently.
     */

    // || drop(v) // FnOnce
    // || v.push(1) // FnMut
    // |args| v.contains(arg) // Fn
    // let y = 5;
    // let add_y = |x| x + y;
    // let copy = add_y;
    // println!("copy(1) = {}", add_y(copy(10)));

    //let mut y = 5;
    //let mut add_y = |x| {y += x; y};
    //let mut copy = add_y;
    //println!("copy(1) = {}", add_y(copy(10)));

    // Iterators
    //let vec = vec![1, 2, 3];
    //for val in vec.iter() {
    //    println!("Got: {}", val);
    //}

    let vec2 = vec![1, 2, 3];
    let mut iter = (&vec2).into_iter();
    while let Some(v) = iter.next() {
        println!("Got: {}", v);
    }

    let mut vec: Vec<Item> = Vec::new();
    vec.push(Item {
        name: "coat".to_string(),
    });
    vec.push(Item {
        name: "book".to_string(),
    });
    vec.push(Item {
        name: "shirt".to_string(),
    });
    vec.push(Item {
        name: "shorts".to_string(),
    });
    vec.push(Item {
        name: "shoes".to_string(),
    });

    let check = check_inventory(vec, "coat".to_string());
    println!("check = {:?}", check);

    let mut range = Range { start: 0, end: 10 };
    //for r in range {
    //    println!("r = {}", r);
    //}

    let vec: Vec<u32> = range.filter(|x| x % 2 == 0).collect();
    println!("vec = {:?}", vec);
}

pub trait Iterators {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    //many default methods
}
