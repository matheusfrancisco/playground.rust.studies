//! ## Exercise 05: Borrow checker and conflicting borrows
//!
//! **Concept:** You cannot have a mutable borrow while an immutable borrow is active, or two
//! mutable borrows of the same data. The borrow checker enforces this at compile time.
//!
//! **Your task:** The function `sum_first_and_last` should return the sum of the first and last
//! elements of a non-empty slice. A naive approach might borrow immutably and then mutably (or
//! vice versa) and get a compile error. Implement it in a way that satisfies the borrow checker
//! (e.g. use indices and direct access, or split the slice).

/// Returns the sum of the first and last element of a non-empty slice of integers.
/// Your implementation must not cause overlapping mutable and immutable borrows.
pub fn sum_first_and_last(slice: &[i32]) -> i32 {
    return slice[0] + slice[slice.len() - 1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_elements() {
        let v = vec![10, 20];
        assert_eq!(sum_first_and_last(&v), 30);
    }

    #[test]
    fn many_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_first_and_last(&v), 6);
    }
}
