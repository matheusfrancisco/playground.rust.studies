use std::collections::{HashMap, HashSet};

fn main() {
    //let m: Vec<_> = vec![1, 2, 3].iter().map(|x| x * 5).collect();
    //println!("{:?}", m);
    let data = vec![1, 2, 3];
    let mut foo = data.iter().map(|x| x * 5);
    let mut new_vec = vec![];

    while let Some(x) = foo.next() {
        new_vec.push(x);
    }

    println!("{:?}", new_vec);

    let bar: String = vec!["this", "to", "collect"].into_iter().collect();
    println!("{:?}", bar);

    let bar: HashSet<isize> = vec![1, 2, 3].into_iter().collect();
    println!("{:?}", bar);

    let foo: HashMap<&str, usize> = vec!["this", " is", "a", "test"]
        .into_iter()
        .enumerate()
        .map(|(idx, item)| (item, idx))
        .collect();
    println!("{:?}", foo);

    let what = vec![1, 2, 3, 4, 5].iter().filter(|x| *x % 2 == 0).count();
    println!("{:?}", what);

    let map = HashMap::from([("foo", 1), ("fb", 1)]);

    println!("{:?}", map);
    map.iter().for_each(|(k, v)| println!("{}: {}", k, v));

    vec![1, 2, 3, 4, 5]
        .iter()
        .skip(2)
        .take_while(|&&x| x >= 3)
        .for_each(|x| println!("i: {}", x));
}
