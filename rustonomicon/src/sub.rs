pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strtok() {
        let mut x = "hello world";
        // strtok<'a, 'b>(&'a mut &'b str, char) -> &'b str
        // strtok        (&'a mut &'static str, char) -> &'static str
        let hello: _ = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}

// T:U
//
//```
//F is covariant if F<Sub> is a subtype of F<Super> (subtyping "passes through")
//F is contravariant if F<Super> is a subtype of F<Sub> (subtyping is "inverted")
//```
//This is essentially saying that
//- For covariant, the most useful type is `<sub>` (the longest lifetime)
//- For contravariant, the most useful type is `<super>` (the shortest lifetime)
//
//So, contravariant and covariant have a nice interplay together between them. For example in `&'a T` we can see `'a` is covariant, hence the most useful type is the longest lifetime, whereas in `fn(T) -> U`, we can see that for `T`, the most useful type is the shortest lifetime (contravariant; so the function can accept any argument since it is asking for the shortest one), hence the covariant's lifetime can be shortened down to whatever needed.
//
//This makes calling function arguments easy since covariants have a longer lifetime, and the
//function expects a shorter lifetime, so we can shorten to whatever is needed
//(which is the entire point of it; so we can pass any longer lived item to a function;
//the function shouldn't care if the reference lives for a shorter time;
//it can forget those details since intuitively the concept holds; 'short is always valid
//for any 'long). I also liked picturing contravariance as `how strict the requirements
//it places are on the caller` (and clearly, contravariance is the least strict on the caller
//since it's asking for the smallest lifetime)
//
//

//fn f(x: &'static str) {
//    println!("{}", x);
//}
//
//fn main() {
//
//    let x = String::from("hello world");
//    f("hello world");
//    //f(&x)
//
//}
