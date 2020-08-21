pub(crate) mod comment;
mod file_entry;
mod line_entry;
mod warning;

pub use file_entry::FileEntry;
pub use line_entry::LineEntry;
pub use warning::Warning;

pub const LF: &str = "\n";

pub fn remove_invalid_leading_chars(string: &str) -> String {
    string
        .chars()
        .skip_while(|&c| !(c.is_alphabetic() || c == '_'))
        .collect()
}

#[test]
fn remove_invalid_leading_chars_test() {
    let string = String::from("-1&*FOO");
    assert_eq!("FOO", remove_invalid_leading_chars(&string));

    let string = String::from("***FOO-BAR");
    assert_eq!("FOO-BAR", remove_invalid_leading_chars(&string));
}
