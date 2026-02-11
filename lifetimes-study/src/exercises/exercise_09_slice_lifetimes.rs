//! ## Exercise 09: Slices and lifetimes (owned vs borrowed)
//!
//! **Concept:** A `String` owns its data; a `&str` borrows it. If a struct owns a `String` and
//! you implement a method that returns a `&str`, that `&str` must borrow from the struct — so
//! its lifetime is tied to `&self`. The compiler enforces that the returned reference cannot
//! outlive the struct.
//!
//! **Your task:** Implement `TextHolder::slice`. The struct owns a `String`; the method returns
//! a string slice (substring) from byte index `start` to `end`. The returned `&str` must have
//! a lifetime tied to `self` (the borrow comes from `self.content`). Use string slice indexing;
//! you may assume valid UTF-8 and that `start`/`end` are on character boundaries, or use
//! `.get(start..end)` to return `Option<&str>` for safety.

/// Holds owned string content. Methods can return string slices that borrow from this.
pub struct TextHolder {
    content: String,
}

impl TextHolder {
    pub fn new(s: String) -> Self {
        TextHolder { content: s }
    }

    /// Returns the substring from byte index `start` to `end` (exclusive).
    /// The returned &str borrows from self, so it must not outlive self.
    /// Return None if the range is invalid (out of bounds or not on UTF-8 boundaries).
    pub fn slice(&self, start: usize, end: usize) -> Option<&str> {
        self.content.get(start..end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_middle() {
        let t = TextHolder::new("hello world".to_string());
        assert_eq!(t.slice(0, 5), Some("hello"));
        assert_eq!(t.slice(6, 11), Some("world"));
    }

    #[test]
    fn slice_invalid_range() {
        let t = TextHolder::new("hi".to_string());
        assert_eq!(t.slice(0, 10), None);
    }
}
