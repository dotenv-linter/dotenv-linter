use std::rc::Rc;

use comment::Comment;

use crate::common::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LineEntry {
    pub number: usize,
    pub file: Rc<FileEntry>,
    pub raw_string: String,

    /// Used in ExtraBlankLineFixer
    pub is_deleted: bool,
}

impl LineEntry {
    pub fn new(number: usize, file: Rc<FileEntry>, raw_string: String, is_deleted: bool) -> Self {
        LineEntry {
            number,
            file,
            raw_string,
            is_deleted,
        }
    }

    pub fn is_empty_or_comment(&self) -> bool {
        self.is_empty() || self.is_comment()
    }

    pub fn is_empty(&self) -> bool {
        self.trimmed_string().is_empty()
    }

    pub fn is_comment(&self) -> bool {
        self.trimmed_string().starts_with('#')
    }

    pub fn get_key(&self) -> Option<&str> {
        if self.is_empty_or_comment() {
            return None;
        }

        let stripped = self.stripped_export_string();
        Some(stripped.split('=').next().unwrap_or(stripped))
    }

    pub fn get_value(&self) -> Option<&str> {
        if self.is_empty_or_comment() {
            return None;
        }

        self.raw_string
            .find('=')
            .map(|idx| &self.raw_string[(idx + 1)..])
    }

    pub fn trimmed_string(&self) -> &str {
        self.raw_string.trim()
    }

    fn stripped_export_string(&self) -> &str {
        let trimmed = self.trimmed_string();
        trimmed
            .strip_prefix("export ")
            .map(str::trim)
            .unwrap_or(trimmed)
    }

    pub fn is_last_line(&self) -> bool {
        self.file.total_lines == self.number
    }

    pub fn mark_as_deleted(&mut self) {
        self.is_deleted = true;
    }

    // Maybe we should add the comment field to the LineEntry struct (but this requires some
    // refactoring of the line entries creation)
    // pub control_comment: Option<Comment<'a>>
    pub fn get_control_comment(&self) -> Option<Comment> {
        if !self.is_comment() {
            return None;
        }
        comment::parse(self.raw_string.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::common::tests::*;

    mod is_empty_or_comment {
        use super::*;

        #[test]
        fn run_with_empty_line_test() {
            let input = line_entry(1, 1, "");

            assert_eq!(input.is_empty(), true);
            assert_eq!(input.is_comment(), false);
            assert_eq!(input.is_empty_or_comment(), true);
        }

        #[test]
        fn run_with_comment_line_test() {
            let input = line_entry(1, 1, "# Comment");

            assert_eq!(input.is_empty(), false);
            assert_eq!(input.is_comment(), true);
            assert_eq!(input.is_empty_or_comment(), true);
        }

        #[test]
        fn run_with_not_comment_or_empty_line_test() {
            let input = line_entry(1, 1, "NotComment");

            assert_eq!(input.is_empty(), false);
            assert_eq!(input.is_comment(), false);
            assert_eq!(input.is_empty_or_comment(), false);
        }
    }

    mod get_key {
        use super::*;

        #[test]
        fn empty_line_test() {
            let input = line_entry(1, 1, "");
            let expected = None;

            assert_eq!(expected, input.get_key());
        }

        #[test]
        fn stripped_export_prefix_test() {
            let input = line_entry(1, 1, "export FOO=BAR");
            let expected = Some("FOO");

            assert_eq!(expected, input.get_key());
        }

        #[test]
        fn correct_line_test() {
            let input = line_entry(1, 1, "FOO=BAR");
            let expected = Some("FOO");

            assert_eq!(expected, input.get_key());
        }

        #[test]
        fn line_without_value_test() {
            let input = line_entry(1, 1, "FOO=");
            let expected = Some("FOO");

            assert_eq!(expected, input.get_key());
        }

        #[test]
        fn missing_value_and_equal_sign_test() {
            let input = line_entry(1, 1, "FOOBAR");
            let expected = Some("FOOBAR");

            assert_eq!(expected, input.get_key());
        }
    }

    mod get_value {
        use super::*;

        #[test]
        fn empty_line_test() {
            let input = line_entry(1, 1, "");
            let expected = None;

            assert_eq!(expected, input.get_value());
        }

        #[test]
        fn correct_line_test() {
            let input = line_entry(1, 1, "FOO=BAR");
            let expected = Some("BAR");

            assert_eq!(expected, input.get_value());
        }

        #[test]
        fn correct_line_with_single_quote_test() {
            let input = line_entry(1, 1, "FOO='BAR'");
            let expected = Some("'BAR'");

            assert_eq!(expected, input.get_value());
        }

        #[test]
        fn correct_line_with_double_quote_test() {
            let input = line_entry(1, 1, "FOO=\"BAR\"");
            let expected = Some("\"BAR\"");

            assert_eq!(expected, input.get_value());
        }

        #[test]
        fn line_without_key_test() {
            let input = line_entry(1, 1, "=BAR");
            let expected = Some("BAR");

            assert_eq!(expected, input.get_value());
        }

        #[test]
        fn line_without_value_test() {
            let input = line_entry(1, 1, "FOO=");
            let expected = Some("");

            assert_eq!(expected, input.get_value());
        }

        #[test]
        fn missing_value_and_equal_sign_test() {
            let input = line_entry(1, 1, "FOOBAR");
            let expected = None;

            assert_eq!(expected, input.get_value());
        }
    }

    mod trimmed_string {
        use super::*;

        #[test]
        fn line_without_blank_chars_test() {
            let entry = line_entry(1, 1, "FOO=BAR");

            assert_eq!("FOO=BAR", entry.trimmed_string());
        }

        #[test]
        fn line_with_spaces_test() {
            let entry = line_entry(1, 1, "   FOO=BAR  ");

            assert_eq!("FOO=BAR", entry.trimmed_string());
        }

        #[test]
        fn line_with_tab_test() {
            let entry = line_entry(1, 1, "FOO=BAR\t");

            assert_eq!("FOO=BAR", entry.trimmed_string());
        }
    }

    mod get_control_comment {
        use super::*;

        #[test]
        fn line_with_control_comment_test() {
            let entry = line_entry(1, 1, "# dotenv-linter:off LowercaseKey");
            let comment = entry.get_control_comment();
            assert!(comment.is_some());

            let comment = entry.get_control_comment().expect("comment");
            assert_eq!(comment.is_disabled(), true);
            assert_eq!(comment.checks, vec!["LowercaseKey"]);
        }

        #[test]
        fn line_with_no_comment_test() {
            let entry = line_entry(1, 1, "A=B");
            let comment = entry.get_control_comment();
            assert!(comment.is_none());
        }
    }
}
