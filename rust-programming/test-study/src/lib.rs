#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let r = 2;
        assert_eq!(r, 2);
    }

    #[test]
    fn it_not_eq() {
        let r = 2;
        assert_ne!(r, 43);
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        panic!("Make this test fail");
    }

    #[test]
    #[ignore]
    fn it_ignore() {
        panic!("Make this test fail");
    }

    #[test]
    fn call_simple_dd() {
        assert!(simple_dd())
    }
}

fn simple_dd() -> bool {
    true
}
