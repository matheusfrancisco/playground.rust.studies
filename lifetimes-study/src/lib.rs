struct MyIterator<'a, T> {
    // this can live  as long the struct itself
    slices: &'a [T],
}

impl<'a, T> Iterator for MyIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // get first element
        // set the slices to the rest of the elements
        // return the first element
        let (first, rest) = self.slices.split_first()?;
        self.slices = rest;
        Some(first)
    }
}

struct MyMutableIterator<'iter, T> {
    // this can live  as long the struct itself
    slice: &'iter mut [T],
}

impl<'iter, T> Iterator for MyMutableIterator<'iter, T> {
    type Item = &'iter mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let slice = &mut self.slice;
        let slice = std::mem::replace(slice, &mut []);
        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;
        // get first element
        // set the slices to the rest of the elements
        // return the first element
        Some(first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let collections = vec![1, 2, 3, 4, 5];
        let wrapper = MyIterator {
            slices: &collections,
        };
        for (index, elem) in wrapper.enumerate() {
            assert_eq!(*elem, collections[index]);
        }
    }

    #[test]
    fn it_works_mut() {
        let mut collections = vec![1, 2, 3, 4, 5];
        let wrapper = MyMutableIterator {
            slice: &mut collections[..],
        };

        for (index, elem) in wrapper.enumerate() {
            *elem = *elem + 1;
        }

        assert_eq!(collections.get(0), Some(&2));
    }
}
