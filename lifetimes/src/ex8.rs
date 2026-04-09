use std::borrow::Cow;

fn capitalize<'a>(input: Cow<'a, str>) -> Cow<'a, str> {
    if input.is_empty() {
        return input;
    }
    if input.chars().all(char::is_uppercase) {
        return input;
    }

    let mut cloned = String::with_capacity(input.len());
    cloned.push_str(&input[..1].to_uppercase());
    cloned.push_str(&input[1..]);
    Cow::Owned(cloned)
}

#[test]
fn test_cow() {
    let s1 = "hello".to_string();
    let s2 = "WORLD".to_string();
    let s3 = "hello".to_string();

    let c1 = capitalize(Cow::Borrowed(&s1));
    let c2 = capitalize(Cow::Borrowed(&s2));
    let c3 = capitalize(Cow::Owned(s3));

    assert_eq!(c1, "Hello");
    assert_eq!(c2, "WORLD");
    assert_eq!(c3, "Hello");
}
