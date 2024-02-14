#![feature(allocator_api)]
use std::alloc::System;

//Constructs a new, empty Vec<T, A>.
//The vector will not allocate until elements are pushed onto it.

fn main() {
    let mut vec: Vec<i32, _> = Vec::new_in(System);
}
