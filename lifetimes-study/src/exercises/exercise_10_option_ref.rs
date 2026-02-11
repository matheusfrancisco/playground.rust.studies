//! ## Exercise 10: Option and references with lifetimes
//!
//! **Concept:** `Option<&'a T>` is a type that either holds a reference with lifetime `'a` or
//! is `None`. When you store or return `Option<&T>`, the `T` must live at least as long as the
//! lifetime you assign. This is common in APIs that might or might not return a reference.
//!
//! **Your task:** Implement `find_after` so it returns `Some(&element)` for the first element in
//! `slice` that is strictly greater than `value`, or `None` if there is none. The returned
//! reference is into `slice`, so it must have the same lifetime as the slice.

/// Returns a reference to the first element in `slice` that is strictly greater than `value`,
/// or `None` if no such element exists.
pub fn find_after<'a>(slice: &'a [i32], value: i32) -> Option<&'a i32> {
    let v = slice.iter().find(|&&x| x > value);
    match v {
        Some(value) => Some(value),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_next() {
        let v = vec![1, 3, 5, 7];
        assert_eq!(find_after(&v, 4), Some(&5));
    }

    #[test]
    fn none_when_all_smaller() {
        let v = vec![1, 2, 3];
        assert_eq!(find_after(&v, 10), None);
    }

    #[test]
    fn first_element() {
        let v = vec![10, 20, 30];
        assert_eq!(find_after(&v, 0), Some(&10));
    }
}
