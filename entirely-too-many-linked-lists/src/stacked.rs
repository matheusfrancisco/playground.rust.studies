#[cfg(test)]
mod test {
    #[test]
    fn test_basic_borrows() {
        let mut data = 10;
        let ref1 = &mut data;
        let ref2 = &mut *ref1;
        *ref2 += 1;
        *ref1 += 1;
        assert_eq!(data, 12);
    }

    #[test]
    fn test_basic_borrows_oder_swap() {
        //this is not safety
        //miri catchs the error
        unsafe {
            let mut data = 10;
            let ref1 = &mut data;
            let ref2 = ref1 as *mut _;
            //ordered swap
            *ref1 += 1;
            *ref2 += 1;
            println!("data: {}", data);
            assert_eq!(data, 12);
        }
    }

    #[test]
    fn test_basic_borrows_oder_swap_2() {
        unsafe {
            let mut data = 10;
            let ref1 = &mut data;
            let ptr2 = ref1 as *mut _;
            let ref3 = &mut *ptr2;
            let ptr4 = ref3 as *mut _;

            // Access the first raw pointer first
            *ptr2 += 2;

            // Then access things in "borrow stack" order
            *ptr4 += 4;
            *ref3 += 3;
            *ptr2 += 2;
            *ref1 += 1;

            println!("{}", data);
        }
    }

    #[test]
    fn test_basic_borrows_oder_swap_3() {
        unsafe {
            let mut data = [0; 10];
            let ref1_at_0 = &mut data[0]; // Reference to 0th element
            let ptr2_at_0 = ref1_at_0 as *mut i32; // Ptr to 0th element
            let ptr3_at_1 = ptr2_at_0; //.add(1); // Ptr to 1st element
            //
            *ptr3_at_1 += 3;
            *ptr2_at_0 += 2;
            *ref1_at_0 += 1;

            // Should be [3, 3, 0, ...]
            println!("{:?}", &data[..]);
        }
    }
}
