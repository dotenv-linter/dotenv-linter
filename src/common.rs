pub(crate) mod comment;
mod file_entry;
mod line_entry;
mod output;
mod warning;

pub use file_entry::FileEntry;
pub use line_entry::LineEntry;
pub use output::Output;
pub use warning::Warning;

/// Mode in which the program is run.
pub enum Mode {
    Fix,
    Check,
}

pub const LF: &str = "\n";

pub fn remove_invalid_leading_chars(string: &str) -> String {
    string
        .chars()
        .skip_while(|&c| !(c.is_alphabetic() || c == '_'))
        .collect()
}

pub(crate) mod tests {
    use super::*;
    use std::path::PathBuf;

    #[allow(dead_code)]
    pub fn blank_line_entry(number: usize, total_lines: usize) -> LineEntry {
        LineEntry {
            number,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines,
            },
            raw_string: String::from("\n"),
            is_deleted: false,
        }
    }

    #[allow(dead_code)]
    pub fn line_entry(number: usize, total_lines: usize, raw_string: &str) -> LineEntry {
        LineEntry {
            number,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines,
            },
            raw_string: String::from(raw_string),
            is_deleted: false,
        }
    }

    #[test]
    fn remove_invalid_leading_chars_test() {
        let string = String::from("-1&*FOO");
        assert_eq!("FOO", remove_invalid_leading_chars(&string));

        let string = String::from("***FOO-BAR");
        assert_eq!("FOO-BAR", remove_invalid_leading_chars(&string));
    }
}
