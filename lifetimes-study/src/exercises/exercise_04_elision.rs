//! ## Exercise 04: Lifetime elision
//!
//! **Concept:** The compiler can often infer lifetimes using "elision rules": (1) each reference
//! parameter gets its own lifetime; (2) if there is exactly one input lifetime, it is assigned to
//! all output lifetimes; (3) if there are multiple input lifetimes but one is `&self` or `&mut self`,
//! that one is assigned to all output lifetimes. When elision doesn't apply, you must write
//! lifetimes explicitly.
//!
//! **Your task:** The signature already uses an explicit lifetime `'a` so the compiler knows the
//! return value lives as long as the inputs. Implement the function to return the first of the
//! two slices (by position: `a`).

/// Returns the first of the two slices (first = a, second = b).
/// Add explicit lifetime annotation(s) so the compiler knows the return lifetime.
pub fn first_of_two<'a>(a: &'a [i32], b: &'a [i32]) -> &'a [i32] {
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_first() {
        let a = [1, 2, 3];
        let b = [4, 5, 6];
        assert_eq!(first_of_two(&a, &b), &[1, 2, 3]);
    }
}
