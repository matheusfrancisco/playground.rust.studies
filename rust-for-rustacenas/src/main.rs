
struct StrSplit<'s, 'p> {
    delemiter: &'p str,
    document: &'s str,
}

impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;

    fn next(&self) -> Option<Self::Item> {
        todo!()
    }
}

fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        delemiter: &c.to_string(),
        document: s,
    }
    .next()
}

// shared references
// can be read but not modified
// once the compiler reads a shared reference, it cannot be modified until the reference goes out of scope
fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    println!("The sum of {} and {} is {}", input, input, sum);
    assert_eq!(*sum, 2 * *input);
}

//In other words, it assumes that the mutable reference is exclusive.
fn noalias(input: &i32, output: &mut i32) {
    // in this case if the input and the ouput
    // are the same variable, the output will be modified before the input is read, which will lead to a wrong result
    // so rust compiler does not allow
    // since an input of 1 could then result in an output of 3 in a case like noalias(&x, &mut x).
    if *input == 1 {
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }
}

fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay, as *s would be empty:
    // let was = *s; // you cannot move out the value since the caller would still think they owned
    // that value and would free it again at  the end of main leading to a doulbe free.
    // but this is:
    let was = std::mem::take(s); // this leave some valid value behind,  is a good candidate
    // it is equivalent to std::mem::replcae(&mut value, Default::default()); it moves value
    // so is this:
    *s = was;
    // we can exchange values behind &mut:
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}

fn main() {
    let input = 5;
    let mut sum = 0;
    cache(&input, &mut sum);
    let x = 1;
    let mut k = 0;
    noalias(&x, &mut k);
    println!("The value of k is {}", k);

    let mut s = Box::new(42);
    replace_with_84(&mut s);
    println!("The value of s is {}", s);
}
