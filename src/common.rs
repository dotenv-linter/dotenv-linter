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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::checks::Check;
    use std::path::PathBuf;
    use std::rc::Rc;

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
            let expected = expected.map(|e| Warning::new(line, checker.name(), e));

            assert_eq!(result, expected);
        }
    }

    pub fn blank_line_entry(number: usize, total_lines: usize) -> LineEntry {
        LineEntry::new(
            number,
            Rc::new(FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines,
            }),
            "\n",
        )
    }

    pub fn line_entry(number: usize, total_lines: usize, raw_string: &str) -> LineEntry {
        LineEntry::new(
            number,
            Rc::new(FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines,
            }),
            raw_string,
        )
    }

    #[test]
    fn remove_invalid_leading_chars_test() {
        let string = "-1&*FOO";
        assert_eq!("FOO", remove_invalid_leading_chars(string));

        let string = "***FOO-BAR";
        assert_eq!("FOO-BAR", remove_invalid_leading_chars(string));
    }

    /**
    The purpose of this macro is to avoid boilerplate code while defining
    input for tests in `Fix` trait implementations.
    Since fixers work on a sequence of input lines and a set of warnings,
    these tests often have to prepare a `Vec<LineEntry>` and a `Vec<Warning>`
    to pass it to `Fix` methods.

    Using this macro, input lines and warnings can be encoded in a vector
    format, where each entry is a pair of a line and an optional warning
    content. The macro takes care of determining the line numbers, thus
    avoiding hard-coded integers.

    Macro expands to a block whose value is of type
    `(Vec<LineEntry>, Vec<Warning>)`.

    # Examples:
    ```
    // input with 3 lines, 1 warning
    let (lines, warnings) = lines_and_warnings![
        "foO=BAR" => Some(("LowercaseKey","The FOO key should be in uppercase")),
        "Z=Y" => None,
        "" => None,
    ];
    ```

    ```
    // input with 3 lines, 0 warning
    let (lines, warnings) = lines_and_warnings![
        "FOO=BAR" => None,
        "Z=Y" => None,
        "" => None,
    ];
    ```
    */
    #[cfg(test)]
    #[macro_export]
    macro_rules! lines_and_warnings {
        (
            $(
                // each repeat must contain `expr => expr`
                $line:expr => $opt_warning:expr
            ),* $(,)*
            // ...zero or more, separated by commas
        ) => {
            // replace with multi-line statment block
            {
                let lines_input = vec![ $( $line ),* ];
                let warnings_input = vec![ $( $opt_warning ),* ];
                let total_lines = lines_input.len();

                let line_entries: Vec<LineEntry> = lines_input
                    .iter()
                    .enumerate()
                    .map(|(i, content)| line_entry(i + 1, total_lines, content))
                    .collect();

                let warnings: Vec<Warning> = warnings_input
                    .iter()
                    .enumerate()
                    .filter_map(|(i, opt_warn)| {
                        opt_warn.and_then(|(kind, msg): (&str, &str)| {
                            Some(Warning::new(line_entries[i].clone(), kind, msg))
                        })
                    })
                    .collect();
                (line_entries, warnings)
            }
        };
    }
}
