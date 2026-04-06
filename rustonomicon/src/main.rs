use std::{collections::HashMap, hash::Hash, ptr::NonNull};

use crate::hrtbs::{Closure, do_it};
mod holder;
mod hrtbs;
mod sub;
mod dropcheck;

struct OwnedRaw<T> {
    ptr: *mut T,
}

impl<T> OwnedRaw<T> {
    fn new(value: T) -> Self {
        let boxed = Box::new(value);
        let ptr = Box::into_raw(boxed);
        Self { ptr }
    }
    fn get(&self) -> &T {
        unsafe {
            // SAFETY:
            // - ptr must be valid
            // - ptr must point to a properly initialized value of type T
            // - ptr must not be null
            // - ptr must remain alive while &self is alive
            &*self.ptr
        }
    }
}

struct OwnedNonNull<T> {
    ptr: NonNull<T>,
}

impl<T> OwnedNonNull<T> {
    fn new(value: T) -> Self {
        let boxed = Box::new(value);

        // Box::into_raw never returns null for a real Box allocation
        let raw = Box::into_raw(boxed);
        let ptr = NonNull::new(raw).unwrap();

        Self { ptr }
    }

    fn get(&self) -> &T {
        unsafe {
            // SAFETY:
            // - ptr came from Box allocation
            // - object is still owned by self
            self.ptr.as_ref()
        }
    }

    fn get_mut(&mut self) -> &mut T {
        unsafe {
            // SAFETY:
            // - &mut self gives exclusive access
            self.ptr.as_mut()
        }
    }
}

impl<T> Drop for OwnedNonNull<T> {
    fn drop(&mut self) {
        unsafe {
            // SAFETY:
            // - ptr came from Box::into_raw exactly once
            // - reconstruct Box exactly once to free it
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}

fn insert(h: &mut HashMap<String, usize>, k: String, v: usize) {
    h.insert(k, v);
}

fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
where
    K: Clone + Eq + Hash,
    V: Default,
{
    // match map.get_mut(&key) {
    //     Some(value) => value,
    //     None => {
    //         map.insert(key.clone(), V::default());
    //         map.get_mut(&key).unwrap()
    //     }
    // }
    map.entry(key).or_default()
}
fn main() {
    let mut h = HashMap::<String, usize>::new();
    get_default(&mut h, "foo".to_string());

    let clo = Closure {
        data: (42, 1),
        func: do_it,
    };
    println!("{}", clo.call());

    let mut x = 10;
    let ptr = NonNull::from(&mut x);
    println!("Before: {}", x);
    println!("HolderNonNull: {}", holder::HolderNonNull::new(ptr).get());

    let raw: *mut i32 = Box::into_raw(Box::new(5));
    let ptr = NonNull::new(raw).unwrap();

    println!("Raw {}", unsafe { *ptr.as_ptr() });
    // printing the memory address of the raw pointer
    println!("Raw pointer address: {:?}", ptr);
}
