enum NavigationAids {
    NDB,
    VOR,
    VORDME,
}

fn main() {
    println!("NDB:\t{}", NavigationAids::NDB as u8);
    println!("VOR:\t{}", NavigationAids::VOR as u8);
    println!("VORDME:\t{}", NavigationAids::VORDME as u8);
    
}


