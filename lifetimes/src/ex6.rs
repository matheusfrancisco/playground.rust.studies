//fn split_at_mut<T>(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
//    let len = self.len();
//    let ptr = self.as_mut_ptr();
//    unsafe {
//        assert!(mid <= len);
//        (
//            slice::from_raw_parts_mut(ptr, mid),
//            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//        )
//    }
//}
//
//

use std::mem;

struct IterMut<'a, T> {
    slice: &'a mut [T],
    //pos: usize
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = mem::take(&mut self.slice);
        if slice.is_empty() {
            return None;
        }

        let (l, r) = slice.split_at_mut(1);
        self.slice = r;
        l.get_mut(0)
    }
}
#[test]
fn ex6() {}
