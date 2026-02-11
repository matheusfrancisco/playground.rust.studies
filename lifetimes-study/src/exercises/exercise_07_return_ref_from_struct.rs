//! ## Exercise 07: Returning a reference from a struct
//!
//! **Concept:** When a method returns a reference that comes from the struct's fields (not from
//! the method's arguments), the return type's lifetime is tied to `&self` or `&mut self`. The
//! compiler will infer that the returned reference cannot outlive the struct reference you called
//! the method on.
//!
//! **Your task:** Implement `get_value` and `get_slice`. The returned references must be tied to
//! the lifetime of the struct's borrowed data (i.e. to `self`).

/// Holds a reference to a slice of bytes. The struct outlives the slice.
pub struct ByteView<'a> {
    data: &'a [u8],
}

impl<'a> ByteView<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ByteView { data }
    }

    /// Returns the byte at index `i`, or `None` if out of bounds.
    /// The returned reference must not outlive `self`.
    pub fn get_value(&self, i: usize) -> Option<&'a u8> {
        match self.data.get(i) {
            Some(value) => Some(value),
            None => None,
        }
    }

    /// Returns the sub-slice from index `start` to `end` (exclusive), or `None` if invalid.
    pub fn get_slice(&self, start: usize, end: usize) -> Option<&'a [u8]> {
        match self.data.get(start..end) {
            Some(slice) => Some(slice),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_value_ok() {
        let bytes = vec![10u8, 20, 30];
        let view = ByteView::new(&bytes);
        assert_eq!(view.get_value(1), Some(&20));
    }

    #[test]
    fn get_value_oob() {
        let bytes = vec![10u8, 20];
        let view = ByteView::new(&bytes);
        assert_eq!(view.get_value(5), None);
    }

    #[test]
    fn get_slice_ok() {
        let bytes = vec![1u8, 2, 3, 4, 5];
        let view = ByteView::new(&bytes);
        assert_eq!(view.get_slice(1, 4), Some(&[2u8, 3, 4][..]));
    }
}
