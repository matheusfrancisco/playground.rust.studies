//Copy trait
// simple types can be a copy type

#[derive(Copy)]
struct Foo {}

struct MyStruct {}

impl Copy for MyStruct {}
impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}
