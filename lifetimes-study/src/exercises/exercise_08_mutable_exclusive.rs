//! ## Exercise 08: Mutable borrows are exclusive
//!
//! **Concept:** At any time, you can have either one mutable reference or any number of immutable
//! references to the same data — not both. A mutable reference is "exclusive": no other reference
//! (mut or not) can exist while it's in use. This prevents data races at compile time.
//!
//! **Your task:** Implement `swap_first_last` so it swaps the first and last element of a
//! non-empty mutable slice. You'll need one mutable borrow; avoid creating a second borrow that
//! would overlap (e.g. don't hold two `&mut` references at once). Use indices and direct
//! assignment, or `slice::split_first_mut` / `split_last_mut`.

/// Swaps the first and last element of the slice. Does nothing if the slice has fewer than 2 elements.
pub fn swap_first_last(slice: &mut [i32]) {
    if slice.len() < 2 {
        return;
    }
    let first = slice[0];
    let last = slice[slice.len() - 1];
    slice[0] = last;
    slice[slice.len() - 1] = first;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_two() {
        let mut v = vec![1, 2];
        swap_first_last(&mut v);
        assert_eq!(v, [2, 1]);
    }

    #[test]
    fn swap_many() {
        let mut v = vec![10, 20, 30, 40];
        swap_first_last(&mut v);
        assert_eq!(v, [40, 20, 30, 10]);
    }

    #[test]
    fn one_element_unchanged() {
        let mut v = vec![42];
        swap_first_last(&mut v);
        assert_eq!(v, [42]);
    }
}
