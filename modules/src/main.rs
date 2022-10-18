mod geo;
use geo::calculations::distance as distance_cal;

fn main() {
    let dst = distance_cal(20.000, 21.9999, -12.233, 111.1111);
    println!("{}", dst);
}

