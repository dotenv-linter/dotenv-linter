pub(crate) mod comment;
mod compare;
mod file_entry;
mod line_entry;
pub(crate) mod output;
mod warning;

use colored::*;
pub use compare::CompareFileType;
pub use compare::CompareWarning;
pub use file_entry::FileEntry;
pub use line_entry::LineEntry;
pub use output::check::CheckOutput;
pub use output::compare::CompareOutput;
pub use output::fix::FixOutput;
pub use warning::Warning;

pub const LF: &str = "\n";

pub fn remove_invalid_leading_chars(string: &str) -> &str {
    string.trim_start_matches(|c: char| !(c.is_alphabetic() || c == '_'))
}

pub(crate) mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::rc::Rc;

    #[allow(dead_code)]
    pub fn blank_line_entry(number: usize, total_lines: usize) -> LineEntry {
        LineEntry::new(
            number,
            Rc::new(FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines,
            }),
            String::from("\n"),
            false,
        )
    }

    #[allow(dead_code)]
    pub fn line_entry(number: usize, total_lines: usize, raw_string: &str) -> LineEntry {
        LineEntry::new(
            number,
            Rc::new(FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines,
            }),
            String::from(raw_string),
            false,
        )
    }

    #[test]
    fn remove_invalid_leading_chars_test() {
        let string = "-1&*FOO";
        assert_eq!("FOO", remove_invalid_leading_chars(string));

        let string = "***FOO-BAR";
        assert_eq!("FOO-BAR", remove_invalid_leading_chars(string));
    }
}
