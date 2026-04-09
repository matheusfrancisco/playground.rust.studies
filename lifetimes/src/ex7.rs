#[derive(Debug)]
pub struct IterMut<'a, T: 'a>(&'a mut [T]);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    //fn next(&mut self) -> Option<Self::Item> {
    //    let s = std::mem::take(&mut self.0);
    //    let (first, rest) = s.split_first_mut()?;
    //    self.0 = rest;
    //    Some(first)
    //}
    fn next(&mut self) -> Option<Self::Item> {
        let s = std::mem::take(&mut self.0);

        if s.is_empty() {
            None
        } else {
            let (first, rest) = s.split_at_mut(1);
            self.0 = rest;
            first.get_mut(0)
        }
    }
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        IterMut(slice)
    }
}

#[test]
fn test_iter_mut() {
    let mut data = [1, 2, 3, 4, 5];
    let mut i = IterMut::new(&mut data);

    // for loop — desugars to calling .next() repeatedly
    for x in &mut i {
        *x *= 2;
    }
}

pub struct IterMutRewindable<'a, T: 'a> {
    len: usize,
    original: *mut [T],
    current: &'a mut [T],
    _marker: std::marker::PhantomData<&'a mut [T]>,
}

impl<'a, T> IterMutRewindable<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        let len = slice.len();
        let original = slice as *mut [T];
        IterMutRewindable {
            len,
            original,
            current: slice,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn rewind(&mut self) {
        // reconstruct the original slice from the raw pointer
        // SAFETY
        // 1. len hasnt changed
        // 2. &mut self gurantees no yielded &mut T are alive
        // 3. original ptr is valid for 'a
        unsafe {
            self.current = &mut *self.original;
        }
    }
}

impl<'a, T> Iterator for IterMutRewindable<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let s = std::mem::take(&mut self.current);
        let (first, rest) = s.split_first_mut()?;
        self.current = rest;
        Some(first)
    }
}


#[test]
fn test_iter_mut_rewindable() {
    let mut data = [1, 2, 3, 4, 5];
    let mut i = IterMutRewindable::new(&mut data);

    for x in &mut i {
        *x *= 2;
    }

    i.rewind();

    for x in &mut i {
        *x += 1;
    }

    assert_eq!(data, [3, 5, 7, 9, 11]);
}
