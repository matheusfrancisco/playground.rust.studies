
struct Boeing {
    required_crew: u8,
    range: u16
}

struct Airbus {
    required_crew: u8,
    range: u16
}


trait Flight {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
}

impl Flight for Boeing {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range+280 > distance) {
            true
        } else {
            false
        }
    }
}

fn main() {
    let b = Boeing {
        required_crew: 4,
        range: 7370
    };

    let airbus = Airbus {
        required_crew: 7,
        range: 5555
    };

    let boeing_is_legal = airbus.is_legal(
        b.required_crew, 18, b.range, 2333
    );
    
}



