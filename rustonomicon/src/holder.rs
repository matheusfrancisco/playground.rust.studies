///Raw pointers
///*const T
///*mut T
/// can be null, can dangle, can be invalid, no aliasing guarantees
/// dereferencing is unsafe
///
///NonNull<T>
///canoot be null, can still dnagle, can still be invalid, no aliasing guarantees by itself
///dereferencing is still unsafe, but it has some safe methods like as_ref and as_mut

pub struct Holder<T> {
    ptr: *const T,
}

pub struct HolderNonNull<T> {
    ptr: std::ptr::NonNull<T>,
}
impl<T> Holder<T> {
    pub fn new(ptr: *const T) -> Self {
        Self { ptr }
    }

    pub fn get(&self) -> &T {
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

impl<T> HolderNonNull<T> {
    pub fn new(ptr: std::ptr::NonNull<T>) -> Self {
        Self { ptr }
    }

    pub fn get(&self) -> &T {
        unsafe {
            // SAFETY:
            // - ptr is non-null by construction
            // - ptr must be valid
            // - ptr must point to a properly initialized value of type T
            // - ptr must remain alive while &self is alive
            self.ptr.as_ref()
        }
    }

    fn get_mut(&mut self) -> &mut T {
        unsafe {
            // SAFETY:
            // - same as above
            // - &mut self gives exclusive access for creating &mut T
            self.ptr.as_mut()
        }
    }
}

use std::ptr::NonNull;

struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
}

struct List<T> {
    head: Option<NonNull<Node<T>>>,
}

impl<T> List<T> {
    fn front(&self) -> Option<&T> {
        let head = self.head?;

        unsafe {
            // SAFETY:
            // - head must point to a valid node
            // - node must live as long as self borrow
            Some(&head.as_ref().value)
        }
    }
}
