struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, new_content: &'a str) -> &str {
        let old_content = self.content;
        self.content = new_content;
        old_content
    }
}

fn main() {
    let mut local_tweet = Tweet { content: "hello" };
    let old_content = local_tweet.replace_content("world");
    println!("old content: {}", old_content);
    println!("new content: {}", local_tweet.content);
}
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime input parameters
// , but one of them is &self or &mut self, the lifetime of
// self is assigned to all output lifetime parameters.
fn take_and_return_contet<'a, 'b>(content: &'a str, content2: &'a str) -> &'a str {
    content
}
