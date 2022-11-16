fn main() {
    //Slices are references to a continguous sequence of elements in a collection

    let tweet = String::from("This is my tweet an it's very very long");

    let trimmed_tweet = &tweet[..20]; //String slices
    let trimmed_tweet_ = &tweet[..]; //String slices
    let trimmed_tweet2 = &tweet[0..]; //String slices

    // String types
    // String growable, heap allocated string (UTF-8 encoded)
    // str -> Immutable sequence of UTF-8 bytes somewhere in memory (stack, heap, or static memory)
    // Handle behing a references (&str) because length of sequence is unknown at compile time

    printnl!("{}", trimmed_tweet);

    // String if you need mutate and pass it to func..

    let s = "my string"; //is -> &str

    let tt = trim_tweet(&tweet);
    let tweet2 = "This is my tweet an it's very very long";

    let tt2 = trim_tweet(&tweet);

    printnl!("{}", tt);
    printnl!("{}", tt2);
    
    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[..3];

    printnl!("{:?}", a_slice)
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20]
}
