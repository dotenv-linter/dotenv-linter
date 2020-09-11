use crate::common::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LineEntry {
    pub number: usize,
    pub file: FileEntry,
    pub raw_string: String,
}

impl LineEntry {
    pub fn is_empty_or_comment(&self) -> bool {
        self.is_empty() || self.is_comment()
    }

    pub fn is_empty(&self) -> bool {
        self.trimmed_string().is_empty()
    }

    pub fn is_comment(&self) -> bool {
        self.trimmed_string().starts_with('#')
    }

    pub fn get_key(&self) -> Option<String> {
        if self.is_empty_or_comment() {
            return None;
        }

        match self.trimmed_string().find('=') {
            Some(index) => Some(self.trimmed_string()[..index].to_owned()),
            None => None,
        }
    }

    pub fn get_value(&self) -> Option<String> {
        if self.is_empty_or_comment() {
            return None;
        }

        match self.raw_string.find('=') {
            Some(index) => Some(self.raw_string[(index + 1)..].to_owned()),
            None => None,
        }
    }

    pub fn trimmed_string(&self) -> &str {
        self.raw_string.trim()
    }

    pub fn is_last_line(&self) -> bool {
        self.file.total_lines == self.number
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
        fn correct_line_test() {
            let input = line_entry(1, 1, "FOO=BAR");
            let expected = Some(String::from("FOO"));

            assert_eq!(expected, input.get_key());
        }

        #[test]
        fn line_without_value_test() {
            let input = line_entry(1, 1, "FOO=");
            let expected = Some(String::from("FOO"));

            assert_eq!(expected, input.get_key());
        }

        #[test]
        fn missing_value_and_equal_sign_test() {
            let input = line_entry(1, 1, "FOOBAR");
            let expected = None;

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
            let expected = Some(String::from("BAR"));

            assert_eq!(expected, input.get_value());
        }

        #[test]
        fn line_without_key_test() {
            let input = line_entry(1, 1, "=BAR");
            let expected = Some(String::from("BAR"));

            assert_eq!(expected, input.get_value());
        }

        #[test]
        fn line_without_value_test() {
            let input = line_entry(1, 1, "FOO=");
            let expected = Some(String::from(""));

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
}
