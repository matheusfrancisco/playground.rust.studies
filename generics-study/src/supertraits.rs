trait Vehicle: Paint{
  fn park(&self);
  fn get_default_color() -> String {
    "red".to_owned()
  }
}

trait Paint {
  fn paint(&self, color: String) {
    println!("Painting {}", color);
  }
}

struct VehicleInfo {
  make: String,
  model: String,
  year: u32,
}

struct Car {
  info: VehicleInfo,
}

struct House { }

impl Vehicle for Car {
  fn park(&self) {
    println!("Parking car");
  }
}

impl Paint for House {}


fn creating_paintable_object (vehicle: bool) ->  Box<dyn Paint> {
  if vehicle {
    Box::new(Car {
      info: VehicleInfo {
        make: "Ford".to_owned(),
        model: "F-150".to_owned(),
        year: 1999,
      }
    })
  } else {
    Box::new(House {})
  }
}

fn main () {
  let obj  = creating_paintable_object(true);
}
