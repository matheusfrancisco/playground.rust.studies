use std::collections::HashSet;

fn main() {
    let mut hs = HashSet::new();
    hs.insert(1);
    hs.insert(2);
    hs.insert(3);
    hs.insert(4);
    hs.insert(4);

    for x in hs.iter() {
        println!("Iter {}", x);
    }

    println!("------------------");
    hs.remove(&4);
    for x in hs.iter() {
        println!("Iter {}", x);
    }

    let mut hs2 = HashSet::new();
    hs2.insert(1);
    hs2.insert(3);
    hs2.insert(5);
    hs2.insert(7);
    println!("------------------");

    for x in hs2.iter() {
        println!("Iter {}", x);
    }

    println!("------------------");
    for x in hs.intersection(&hs2) {
        println!("Intersection {}", x);
    }

    let intersection = &hs & &hs2;
    println!("Intersection {:?}", intersection);
    println!("------------------");

    let union = &hs | &hs2;

    println!("Union {:?}", union);
    for x in union {
        println!("Union {}", x);
    }

    println!("------------------");
    let difference = &hs - &hs2;

    println!("Difference {:?}", difference);
}
