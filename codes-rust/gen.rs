use std::ops::{Add, Sub};

#[derive(Debug)]
struct NdbNavAid {
    name: String,
    frenquecy: u16
}

#[derive(Debug)]
struct VorNavAid {
    name: String,
    frenquecy: f32
}

#[derive(Debug)]
struct NdbNavAid1<T, U> {
    name: String,
    frenquecy: T,
    data: U
}


fn main() {

    let vor = NdbNavAid1 {
        name: String::from("DQN"),
        frenquecy: 114.5,
        data: String::from("DQN is VOR")
    };

    let ndb_data:Option<String> = Option::None;
    let ndb = NavAid {
        name: String::from("HKF"),
        frenquecy: 239,
        data: ndb_data
    };

    let s = add(254, 234);
    
}

fn add<T: Add<Output = T>>(op1: T, op2: T) -> T {
    op1 + op2
}

fn add2<T>(op1: T, op2: T) -> T 
where T: Add<Output = T> + Sub<Output = T>
{
    op1 + op2
}
