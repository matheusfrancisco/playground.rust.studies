/// Rust requires any _references_ to freeze:
/// - the referent and its owners.
///
/// While a _reference_ is **in scope**, Rust will not allow you to:
/// - change the referent and its owners.
///
/// [More info](https://doc.rust-lang.org/nomicon/ownership.html)
#[test]
fn ex_1_references() {
    #[allow(dead_code)]
    fn try_to_use_after_free(arg: usize) -> &'static str {
        let s = format!("{} is a number", arg);
        // return &s; 
        unreachable!()
    }

    fn try_to_modify_referent() {
        let mut data = vec![1, 2, 3]; /* referent */
        let ref_to_first_item = &data[0]; /* reference */
        // data.push(4); 
        println!("first_item: {}", ref_to_first_item); /* reference still in scope */
        // drop(ref_to_first_item);
    }
}
