mod compare;
mod file_entry;
mod line_entry;
mod warning;

pub(crate) mod comment;
pub(crate) mod lint_kind;
pub(crate) mod output;

pub use compare::CompareFileType;
pub use compare::CompareWarning;
pub use file_entry::FileEntry;
pub use line_entry::LineEntry;
pub use lint_kind::LintKind;
pub use output::check::CheckOutput;
pub use output::compare::CompareOutput;
pub use output::fix::FixOutput;
pub use warning::Warning;

pub const LF: &str = "\n";

pub fn remove_invalid_leading_chars(string: &str) -> &str {
    string.trim_start_matches(|c: char| !(c.is_alphabetic() || c == '_'))
}

pub fn is_escaped(prefix: &str) -> bool {
    prefix.chars().rev().take_while(|ch| *ch == '\\').count() % 2 == 1
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::checks::Check;

    /**
        Helper function for testing `Check` implementations.

        A `Check` implementation can be used against a number of &str inputs
        and optional `Warning` messages respectively.

        This function construct `LineEntry`s and optional `Warning`s,
        if required, in order to assert that the `Check` implementation is creating
        the correct `Warning` and not just the correct message.

        # Example

        ```no_run
        #[test]
        fn with_one_duplicated_test_key() {
            check_test(&mut DuplicatedKeyChecker::default(),
                [
                    ("FOO=BAR", None),
                    ("FOO=BAR", Some("The FOO key is duplicated")),
                ],
            );
        }
        ```
        The above will assert that on the first line "FOO=BAR" does not cause
        any warnings, hence the `None`.

        The second line however, should expect a `Warning` with a message of
        "The FOO key is duplicated".
    */
    pub fn check_test<'test, T, U>(checker: &mut T, asserts: U)
    where
        T: Check,
        U: AsRef<[(&'test str, Option<&'test str>)]>,
    {
        let asserts = asserts.as_ref();
        let mut line_number = 1;
        let total = asserts.len();

        for (input, expected) in asserts {
            let line = line_entry(line_number, total, input);
            line_number += 1;

            let result = checker.run(&line);
            let expected = expected.map(|e| Warning::new(line.number, checker.name(), e));

            assert_eq!(result, expected);
        }
    }

    pub fn blank_line_entry(number: usize, total_lines: usize) -> LineEntry {
        LineEntry::new(number, "\n", total_lines == number)
    }

    pub fn line_entry(number: usize, total_lines: usize, raw_string: &str) -> LineEntry {
        LineEntry::new(number, raw_string, total_lines == number)
    }

    #[test]
    fn remove_invalid_leading_chars_test() {
        let string = "-1&*FOO";
        assert_eq!("FOO", remove_invalid_leading_chars(string));

        let string = "***FOO-BAR";
        assert_eq!("FOO-BAR", remove_invalid_leading_chars(string));
    }

    #[test]
    fn is_escaped_value_test() {
        let escaped = "\\";
        assert!(is_escaped(escaped));

        let escaped = "\\\\\\";
        assert!(is_escaped(escaped));

        let non_escaped = "\\\\";
        assert!(!is_escaped(non_escaped));

        let random_string = "text without escaping";
        assert!(!is_escaped(random_string));
    }
}
