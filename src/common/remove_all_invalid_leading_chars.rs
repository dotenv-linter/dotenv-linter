pub fn remove_all_invalid_leading_chars(string: &str) -> String {
    string
        .chars()
        .skip_while(|c| !(c.is_alphabetic() || *c == '_'))
        .collect()
}

#[test]
fn remove_all_invalid_leading_chars_test() {
    let string = String::from("-1&*FOO");
    assert_eq!("FOO", remove_all_invalid_leading_chars(&string));

    let string = String::from("***FOO-BAR");
    assert_eq!("FOO-BAR", remove_all_invalid_leading_chars(&string));
}
