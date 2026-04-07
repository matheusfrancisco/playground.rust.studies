/// We can use lifetimes and slices to work with data without modifying it. This pattern
/// shows up a lot when working with parsers (eg: `nom`) and general string manipulation.

fn middle_word1<'a>(input: &'a str) -> &'a str {
    //&words[1]
    //let iter = input.split_whitespace();
    let words: Vec<&str> = input.split(' ').collect();
    let l = words.len();
    let mid = l / 2;
    &words[mid]
}

fn middle_word<'input>(input: &'input str) -> &'input str {
    let iter = input.split_whitespace();

    let (_, middle_word_index) = {
        let iter_clone = iter.clone();
        let word_count = iter_clone.count();
        let middle_word_index = word_count / 2;
        (word_count, middle_word_index)
    };

    let (middle_word_len, len_until_middle_word) = {
        let mut middle_word_len = 0;
        let len_until_middle_word = iter
            .enumerate()
            // Go as far as the middle word.
            .take_while(|(index, _)| *index <= middle_word_index)
            .map(|(index, word)| {
                // At middle word.
                if index == middle_word_index {
                    middle_word_len = word.len();
                    0
                }
                // Before middle word.
                else {
                    word.len()
                }
            })
            .sum::<usize>();

        (middle_word_len, len_until_middle_word)
    };

    let (start_index, end_index) = {
        let start_index = len_until_middle_word + 1;
        let end_index = len_until_middle_word + middle_word_len + 1;
        (start_index, end_index)
    };

    &/*'input*/input[start_index..end_index]
}

#[rustfmt::skip]
#[test]
fn ex_4_input_slices() {
    // 'fn {
        let data = String::from("foo bar baz");
        let middle_word: & /*'fn*/ str = middle_word(&data);
        assert_eq!(middle_word, "bar");
    // }
}
