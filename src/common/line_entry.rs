use crate::common::*;
use comment::Comment;

#[derive(Clone, Debug, PartialEq)]
pub struct LineEntry {
    pub number: usize,
    pub file: FileEntry,
    pub raw_string: String,

    /// Used in ExtraBlankLineFixer
    pub is_deleted: bool,
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

    #[allow(dead_code)]
    pub fn get_substitution_keys(&self) -> Vec<&str> {
        let mut keys = Vec::new();

        let mut value = match self.get_value().map(str::trim) {
            Some(value) if !value.starts_with('\'') => value,
            _ => return keys,
        };

        let is_escaped =
            |prefix: &str| prefix.chars().rev().take_while(|ch| *ch == '\\').count() % 2 == 1;

        if value.starts_with('\"') {
            if value.ends_with('\"') && !is_escaped(&value[..value.len() - 1]) {
                value = &value[1..value.len() - 1]
            } else {
                return keys;
            }
        }

        while let Some(index) = value.find('$') {
            let prefix = &value[..index];
            let raw_key = &value[index + 1..];

            if is_escaped(prefix) {
                value = &raw_key;
            } else {
                let (key, rest) = raw_key
                    .strip_prefix('{')
                    .and_then(|raw_key| raw_key.find('}').map(|i| raw_key.split_at(i)))
                    .or_else(|| {
                        raw_key
                            .find(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                            .map(|i| raw_key.split_at(i))
                    })
                    .unwrap_or((raw_key, ""));
                if !key.is_empty() {
                    keys.push(key);
                } else {
                    return keys;
                }

                value = rest;
            }
        }
        keys
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

    mod get_substitution_keys {
        use super::*;

        #[test]
        fn run_with_empty() {
            let input = line_entry(1, 1, "");
            assert!(input.get_substitution_keys().is_empty());
        }

        #[test]
        fn run_with_simple() {
            let input = line_entry(1, 1, "FOO=$BAR");
            assert_eq!(input.get_substitution_keys(), vec!["BAR"]);
        }

        #[test]
        fn run_with_simple_comment() {
            let input = line_entry(1, 1, "FOO=$BAR # comment");
            assert_eq!(input.get_substitution_keys(), vec!["BAR"]);
        }

        #[test]
        fn run_with_curly_braces() {
            let input = line_entry(1, 1, "FOO=${BAR}");
            assert_eq!(input.get_substitution_keys(), vec!["BAR"]);

            let input = line_entry(1, 1, "FOO=$BAR}");
            assert_eq!(input.get_substitution_keys(), vec!["BAR"]);

            let input = line_entry(1, 1, "FOO=${BAR");
            assert!(input.get_substitution_keys().is_empty());
        }

        #[test]
        fn run_with_double_quotes() {
            let input = line_entry(1, 1, r#"FOO="$BAR""#);
            assert_eq!(input.get_substitution_keys(), vec!["BAR"]);

            let input = line_entry(1, 1, r#"FOO=$BAR""#);
            assert_eq!(input.get_substitution_keys(), vec!["BAR"]);

            let input = line_entry(1, 1, r#"FOO="$BAR"#);
            assert!(input.get_substitution_keys().is_empty());

            let input = line_entry(1, 1, r#"FOO="$BAR\""#);
            assert!(input.get_substitution_keys().is_empty());

            let input = line_entry(1, 1, r#"FOO="\""#);
            assert!(input.get_substitution_keys().is_empty());

            let input = line_entry(1, 1, r#"FOO="${BAR}\\""#);
            assert_eq!(input.get_substitution_keys(), vec!["BAR"]);
        }

        #[test]
        fn run_with_single_quotes() {
            let input = line_entry(1, 1, "FOO='$BAR'");
            assert!(input.get_substitution_keys().is_empty());

            let input = line_entry(1, 1, r"FOO=TEST_${BAR}_\'");
            assert_eq!(input.get_substitution_keys(), vec!["BAR"]);
        }

        #[test]
        fn run_with_escaped_dollar() {
            let input = line_entry(1, 1, r"FOO=\$BAR");
            assert!(input.get_substitution_keys().is_empty());

            let input = line_entry(1, 1, r"FOO=\\$BAR");
            assert_eq!(input.get_substitution_keys(), vec!["BAR"]);

            let input = line_entry(1, 1, r"FOO=\\\$BAR");
            assert!(input.get_substitution_keys().is_empty());
        }

        #[test]
        fn run_with_complicated() {
            let input = line_entry(1, 1, "DATABASE=postgres://${USER}@localhost/database");
            assert_eq!(input.get_substitution_keys(), vec!["USER"]);
        }

        #[test]
        fn run_with_reused() {
            let input = line_entry(1, 1, "FOO=$BAR$BAR");
            assert_eq!(input.get_substitution_keys(), vec!["BAR", "BAR"]);

            let input = line_entry(1, 1, "FOO=${BAR}${BAR}");
            assert_eq!(input.get_substitution_keys(), vec!["BAR", "BAR"]);

            let input = line_entry(1, 1, "FOO=${BAR}${BAZ}");
            assert_eq!(input.get_substitution_keys(), vec!["BAR", "BAZ"]);
        }

        #[test]
        fn run_with_break() {
            let input = line_entry(1, 1, "FOO=${BAR $BAZ");
            assert!(input.get_substitution_keys().is_empty());
        }
    }
}
