trait Park {
  fn park(&self);
}

trait Paint {
  fn paint(&self) {
    println!("Painting vehicle");
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

struct Truck {
  info: VehicleInfo,
}

struct House { }

impl Truck {
  fn unload(&self) {
    println!("Unloading truck");
  }
}

impl Park for Car {
  fn park(&self) {
    println!("Parking car");
  }
}

impl Park for Truck {
  fn park(&self) {
    println!("Parking truck");
  }
}

impl Paint for Car {
  fn paint(&self, color: String) {
    println!("Painting car {} {}", color, self.info.model);
  }
}
 impl Paint for Truck {
  fn paint(&self, color: String) {
    println!("Painting truck {} {}", color, self.info.model);
  }
}

impl Paint for House {
  fn paint(&self, color: String) {
    println!("Painting house {}", color);
  }
}

fn main() {
  let car = Car {
    info: VehicleInfo {
      make: "Toyota".to_string(),
      model: "Camry".to_string(),
      year: 2014,
    },
  };

  car.unload();
  car.park();
}

fn paint_red<T>(object: &T) {
  object.paint("red".to_owned());
}

fn paint_red2<T>(object: &impl Paitn) {
  object.paint("red".to_owned());
}

fn paint_red3<T: Paint>(object: &T) {
  object.paint("red".to_owned());
}

fn paint_red4<T>(object: &T) where T: Paint {
  object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(obj: &T) where T: Paint + Park {
  obj.paint("red".to_owned());
  obj.park();
}

fn create_paintable_obj() -> impl Paint {
  House { }
}


