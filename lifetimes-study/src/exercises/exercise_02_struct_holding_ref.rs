//! ## Exercise 02: Struct holding a reference
//!
//! **Concept:** If a struct holds a reference, the struct must not outlive the data it references.
//! We give the struct a lifetime parameter: `struct Holder<'a> { data: &'a T }`. Every reference
//! inside the struct is tied to that same `'a`; the struct is only valid while `'a` is valid.
//!
//! **Your task:** Add the correct lifetime parameter to `Excerpt` so it compiles. The field
//! `first_sentence` must not outlive the borrowed content.

/// A struct that holds a reference to a string slice.
/// Add a lifetime parameter so the struct is valid only while the slice is valid.
pub struct Excerpt<'a> {
    pub first_sentence: &'a str,
}

impl<'a> Excerpt<'a> {
    /// Creates an excerpt from the given string: the first sentence (up to the first '.').
    pub fn new(text: &'a str) -> Self {
        let end = text.find('.');
        let first_sentence = match end {
            Some(i) => &text[..=i],
            None => text,
        };
        Excerpt { first_sentence }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_sentence() {
        let novel = String::from("Call me Ishmael.");
        let excerpt = Excerpt::new(&novel);
        assert_eq!(excerpt.first_sentence, "Call me Ishmael.");
    }

    #[test]
    fn first_of_many() {
        let novel = String::from("First. Second. Third.");
        let excerpt = Excerpt::new(&novel);
        assert_eq!(excerpt.first_sentence, "First.");
    }
}
