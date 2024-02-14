enum NavigationAids {
    NDB(u16),
    VOR(String, f32),
    VORDME(String, f32),
    FIX {
        name: String,
        latitude: f32,
        longitude: f32,
    },
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is {:?}", six);
    println!("none is {:?}", none);

    // six is Some(6)
    // none is None
    let ndb_freq: u16 = 384;

    match ndb_freq {
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
            println!("NDB Frequency is valid");
        }
        _ => {
            println!("NDB Frequency is not valid");
        }
    }

    let ndb_uwl = NavigationAids::NDB(385);
    let vor_dqn = NavigationAids::VOR(String::from("DQN"), 114.5);
    let vor_dme_sgh = NavigationAids::VORDME(String::from("SGH"), 113.2);

    let fix_tarry = NavigationAids::FIX {
        name: String::from("TARRY"),
        latitude: 40.05333,
        longitude: -83.91367,
    };

    print_nav_aid(&ndb_uwl);
    print_nav_aid(&vor_dqn);
    print_nav_aid(&vor_dme_sgh);
    print_nav_aid(&fix_tarry);
}

fn plus_one(x: Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_nav_aid(navaid: &NavigationAids) {
    match navaid {
        NavigationAids::NDB(khz) => {
            println!("NDB frequency is {} kilohertz", khz);
        }
        NavigationAids::VOR(id, freq) => {
            println!(
                "VOR identifier is {} and it is frequency is {} kilohertz",
                id, freq
            );
        }
        NavigationAids::VORDME(id, freq) => {
            println!(
                "VORME identifier is {} and it is frequency is {} kilohertz",
                id, freq
            );
        }
        NavigationAids::FIX {
            name,
            latitude,
            longitude,
        } => {
            println!(
                "FIX {} is at {} latitude and {} longitude",
                name, latitude, longitude
            );
        }
    }
}
