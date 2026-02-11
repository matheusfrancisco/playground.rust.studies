//! ## Exercise 06: Lifetime in impl block
//!
//! **Concept:** When implementing methods for a struct that holds references, the `impl` block
//! needs the same lifetime parameter: `impl<'a> Struct<'a>`. Methods that return references
//! tied to the struct's data will use that same `'a`.
//!
//! **Your task:** Add the missing lifetime parameter to the struct and impl block, then
//! implement `prefix` to return a slice of the first `n` bytes (characters) of the stored string.
//! If `n` is greater than the length, return the whole string.

/// A wrapper around a string slice. Add a lifetime parameter so it can hold a reference.
pub struct StrWrapper<'a> {
    inner: &'a str,
}

impl<'a> StrWrapper<'a> {
    /// Creates a new wrapper around the given string slice.
    pub fn new(s: &'a str) -> Self {
        StrWrapper { inner: s }
    }

    /// Returns the prefix of the inner string of length `n` (or the whole string if shorter).
    pub fn prefix(&self, n: usize) -> &'a str {
        &self.inner[..n.min(self.inner.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_short() {
        let s = "hello";
        let w = StrWrapper::new(s);
        assert_eq!(w.prefix(2), "he");
    }

    #[test]
    fn prefix_long() {
        let s = "hi";
        let w = StrWrapper::new(s);
        assert_eq!(w.prefix(10), "hi");
    }
}
