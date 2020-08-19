pub fn remove_all_invalid_leading_chars(string: &str) -> String {
    let mut cleaned_string = string;

    while !cleaned_string.starts_with(|c: char| c.is_alphabetic() || c == '_') {
        cleaned_string = &cleaned_string[1..]
    }

    cleaned_string.to_string()
}

#[test]
fn remove_all_invalid_leading_chars_test() {
    let string = String::from("-1&*FOO");
    assert_eq!("FOO", remove_all_invalid_leading_chars(&string));

    let string = String::from("***FOO-BAR");
    assert_eq!("FOO-BAR", remove_all_invalid_leading_chars(&string));
}
