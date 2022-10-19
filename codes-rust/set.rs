use std::collections::HashSet;

fn main() {
    let mut f = HashSet::new();

    f.insert(("DA918", "11", "Orlando"));
    f.insert(("DA228", "1", "Orldo"));
    f.insert(("DA218", "11", "Oro"));

    for ff in f.iter() {
        println!("{:?}", ff);
    }

}
