
/*Structs and lifetime
*https://doc.rust-lang.org/book/ch05-03-method-syntax.html
* */

#[derive(Debug)]
struct Square {
    width: i32,
    height: i32,
}

impl Square {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn whats_my_width(&self) -> i32 {
        self.width
    }

    fn change_width(&mut self, width: i32) {
        self.width = width;
    }
}

fn main() {
    let square = Square {
        width: 10,
        height: 10,
    };
    println!("Area of square is {}", square.area());
    println!("Width of square is {}", square.whats_my_width());

    dbg!(&square);

    let mut square = Square {
        width: 10,
        height: 10,
    };
    square.change_width(20);
    println!("Width of square is {}", square.whats_my_width());
/*
  * Where’s the -> Operator?
In C and C++, two different operators are used for calling methods: you use . 
  if you’re calling a method on the object directly and -> if you’re calling 
  the method on a pointer to the object and need to dereference the pointer first. 
  In other words, if object is a pointer, object->something() is similar to (*object).something().

Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called 
  automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.

Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, 
  &mut, or * so object matches the signature of the method. In other words, the following are the same:

p1.distance(&p2);
(&p1).distance(&p2);
The first one looks much cleaner. This automatic referencing behavior works because methods have
  a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out 
  definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). 
  The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.*/
}
