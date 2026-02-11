//! ## Exercise 01: Basic lifetime annotation
//!
//! **Concept:** When a function returns a reference, that reference must come from one of the
//! inputs. The compiler needs to know how the return value's lifetime relates to the inputs'
//! lifetimes. We use a single lifetime parameter `'a` to say "the returned reference lives as
//! long as the shorter of the two inputs."
//!
//! **Your task:** Implement `longest` so it returns the longer of the two string slices.
//! The lifetime annotation `'a` is already in place: the returned `&str` must not outlive
//! either `a` or `b`.

/// Returns the longer of two string slices.
/// The returned reference has lifetime `'a`, meaning it is valid as long as both inputs are.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_longer() {
        let a = "hello";
        let b = "hi";
        assert_eq!(longest(a, b), "hello");
    }

    #[test]
    fn second_longer() {
        let a = "hi";
        let b = "hello";
        assert_eq!(longest(a, b), "hello");
    }

    #[test]
    fn equal_length() {
        let a = "abc";
        let b = "xyz";
        assert_eq!(longest(a, b), "abc"); // or "xyz", either is correct
    }
}
