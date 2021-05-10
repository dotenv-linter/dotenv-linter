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
    use crate::fixes::Fix;
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
    `TestLine` is a helper type to prepare a line input for `Fix`
    implementation tests. Since fixers work on a sequence of input
    lines and a set of warnings, these tests require preparing line
    entries and warnings mapping to those entries. `TestLine` make it
    easier to define a test line entry with zero or more warnings
    corresponding to the entry, in a manner independent of `LineEntry`
    and `Warning` definitions.

    Example of usage:
    ```
    let lines = vec![
       // line with no warning
       TestLine::new("A=Foo"),

       // line with one warning
       TestLine::new("a=Foo").warning("LowercaseKey", "The a key should be in uppercase"),

       // line with two warnings
       TestLine::new("a")
            .warning("LowercaseKey", "The a key should be in uppercase")
            .warning(
                "KeyWithoutValue",
                "The a key should be with a value or have an equal sign",
            ),
    ];
    ```
    */
    pub struct TestLine<'l, 'w> {
        line: &'l str,
        warnings: Vec<(&'w str, &'w str)>,
    }

    impl<'l, 'w> TestLine<'l, 'w> {
        pub fn new(line: &'l str) -> Self {
            TestLine {
                line,
                warnings: Vec::new(),
            }
        }

        pub fn warning(mut self, name: &'w str, msg: &'w str) -> Self {
            self.warnings.push((name, msg));
            self
        }
    }

    // Helper type to define a sequence of TestLines. It provides a
    // method to prepare `Vec<LineEntry>` and `Vec<Warning>` output out of
    // test lines input.
    pub struct TestLineEntries<'l, 'w> {
        test_lines: Vec<TestLine<'l, 'w>>,
    }

    impl<'l, 'w> TestLineEntries<'l, 'w> {
        pub fn new(test_lines: Vec<TestLine<'l, 'w>>) -> Self {
            TestLineEntries { test_lines }
        }

        pub fn lines_and_warnings(&self) -> (Vec<LineEntry>, Vec<Warning>) {
            let mut line_no = 1;
            let total_lines = self.test_lines.len();

            let mut lines: Vec<LineEntry> = Vec::new();
            let mut warnings: Vec<Warning> = Vec::new();

            for test_line in &self.test_lines {
                let new_entry = line_entry(line_no, total_lines, test_line.line);
                for (w_name, w_msg) in &test_line.warnings {
                    warnings.push(Warning::new(new_entry.clone(), *w_name, *w_msg));
                }
                lines.push(new_entry);
                line_no += 1;
            }
            (lines, warnings)
        }
    }

    impl<'l, 'w> From<Vec<TestLine<'l, 'w>>> for TestLineEntries<'l, 'w> {
        fn from(test_lines: Vec<TestLine<'l, 'w>>) -> Self {
            TestLineEntries::new(test_lines)
        }
    }

    pub fn run_fix_warnings<F: Fix>(
        fixer: &mut F,
        test_lines: TestLineEntries,
    ) -> (Option<usize>, Vec<String>) {
        let (mut lines, mut warnings) = test_lines.lines_and_warnings();

        let warnings_mut = warnings.iter_mut().collect();
        let fix_count = fixer.fix_warnings(warnings_mut, &mut lines);

        // Remove lines marked as deleted
        lines.retain(|l| !l.is_deleted);

        let fixed_lines: Vec<String> = lines.iter().map(|le| le.raw_string.clone()).collect();
        (fix_count, fixed_lines)
    }

    #[test]
    fn test_line_without_warning() {
        let test_line = TestLine::new("A=Foo");
        assert_eq!("A=Foo", test_line.line);
        assert_eq!(0, test_line.warnings.len());
    }

    #[test]
    fn test_line_with_single_warning() {
        let test_line =
            TestLine::new("a=Foo").warning("LowercaseKey", "The a key should be in uppercase");

        assert_eq!("a=Foo", test_line.line);
        assert_eq!(1, test_line.warnings.len());
        assert_eq!(
            ("LowercaseKey", "The a key should be in uppercase"),
            test_line.warnings[0]
        );
    }

    #[test]
    fn test_line_with_multi_warnings() {
        let test_line = TestLine::new("a")
            .warning("LowercaseKey", "The a key should be in uppercase")
            .warning(
                "KeyWithoutValue",
                "The a key should be with a value or have an equal sign",
            );

        assert_eq!("a", test_line.line);
        assert_eq!(2, test_line.warnings.len());
        assert_eq!(
            ("LowercaseKey", "The a key should be in uppercase"),
            test_line.warnings[0]
        );
        assert_eq!(
            (
                "KeyWithoutValue",
                "The a key should be with a value or have an equal sign"
            ),
            test_line.warnings[1]
        );
    }
}
