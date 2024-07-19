use front_of_house::front_of_house::hosting;

mod front_of_house;

pub fn eat_at_restaurant() {
 // Absolute path
  hosting::add_to_waitlist();

  // Relative path
  //front_of_house::hosting::add_to_waitlist();
    
}


fn server_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    super::server_order();
  }

  fn cook_order() {}
}
