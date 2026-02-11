//! ## Exercise 03: Multiple lifetime parameters
//!
//! **Concept:** A function can have references with different lifetime parameters (`'a`, `'b`).
//! The return type's lifetime must be tied to one of the inputs. Here we return a slice from
//! `long`, so the return type is `&'b str` — the returned reference cannot outlive `long`.
//!
//! **Your task:** Return the first word of `long` (slice up to the first space, or the whole
//! string if there is no space). The signature already ties the return value to `long`'s lifetime.

/// Returns the first word of the string slice `long`.
/// The returned reference has lifetime `'b`, so it is valid as long as `long` is valid.
pub fn first_word_of_long<'a, 'b>(_short: &'a str, long: &'b str) -> &'b str {
    long.find(' ').map(|index| &long[..index]).unwrap_or(long)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_word() {
        let short = "x";
        let long = "hello";
        assert_eq!(first_word_of_long(short, long), "hello");
    }

    #[test]
    fn first_of_several() {
        let a = "a";
        let b = "one two three";
        assert_eq!(first_word_of_long(a, b), "one");
    }
}
