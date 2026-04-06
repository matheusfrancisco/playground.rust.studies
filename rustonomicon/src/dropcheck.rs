use std::marker::PhantomData;
struct Foo<T> {
    // some fields
    _t: PhantomData<T>, // "I conceptually own a T"
                        // when the struct owns or produes values of type T
}

struct MyBox<T> {
    ptr: *const T,      // raw pointer -- covariant like &T
    _t: PhantomData<T>, // "tells compiler: I own a T"
}
//* const T alone would make the compiler give you covariance, but PhantomData<T>
// also tells the drop checker that you struct owns a T and should run its destructor
// important for safety

struct Bar<T> {
    _t: PhantomData<fn() -> T>, // "I produce T but don't own it"
}
// e.g
struct Iter<'a, T> {
    ptr: *const T,
    end: *const T,
    _t: PhantomData<&'a T>, // produces &'a T, covariant, but doesn't own T
}


// You're writing a custom allocator that hands out pointers
// You produce T values but the allocator doesn't "own" them
struct Allocator<T> {
    pool: *mut u8,
    _t: PhantomData<fn() -> T>, // produces T, no ownership
}

// You're writing a sink that consumes T values
struct Sink<T> {
    handler: *mut u8,
    _t: PhantomData<fn(T)>, // consumes T, contravariant
}

// You're writing a cache that reads AND writes T
struct Cache<T> {
    data: *mut u8,
    _t: PhantomData<fn(T) -> T>, // invariant — must match exactly
}
struct Deserializer<T> {
    _t: PhantomData<T>,
}
struct Deserializer2<T> {
    _t: PhantomData<fn() -> T>,
}
struct Deserializer3<T> {
    _t: PhantomData<fn(T)>,
}
