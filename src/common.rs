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

    /**
       Creates test cases for Check trait implementations.

       check_tester takes the name of the Check implementation which is
       followed by the test cases.

       Each test case is the same as a test function so multiple inputs
       will occur on the same instance of the Check implementation, allowing
       the implementation to use internal state (if required).

       # Examples

       The following example shows a single test case for the
       DuplicatedKeyChecker:

       ```
       check_tester!{
           DuplicatedKeyChecker;
           duplicated_key_test => {
               // First time seeing key "FOO" so is None
               "FOO=BAR" => None,
               // Second time seeing key "FOO" so is duplicate.
               "FOO=BAR" => Some("The FOO key is duplicated"),
           }
       }
       ```

       The above example could be expanded as such:
       ```
        #[test]
        fn duplicated_key_test() {
            let mut checker = DuplicatedKeyChecker::default();

            assert_eq!(None, checker.run(&line_entry(1, 2, "FOO=BAR")));
            assert_eq!(Some(
                Warning::new(
                    line_entry(2, 2, "FOO=BAR")),
                    "DuplicatedKey",
                    "The FOO key is duplicated"
                ),
                checker.run(&line_entry(2, 2, "FOO=BAR"))
            );
        }
       ```
       As shown in the expanded example this macro will create the correct
       Line_Entry for the checker and will also create the expected Warning
       using the input and expected warning message.

       ## Constant Warning Message

       Note that the expected warning message does not need to be a string
       literal; in the case where the warning message is always the same
       a constant &str can be used:

       ```
        const WARNING: &str = "Invalid leading character detected";

        check_tester!{
            LeadingCharacterChecker;
            leading_dot_is_not_valid => {
                ".FOO=BAR" => Some(WARNING),
            },
            leading_number_is_not_valid => {
                "1FOO=BAR" => Some(WARNING),
            }
        }
       ```
    */
    #[cfg(test)]
    #[macro_export]
    macro_rules! check_tester {
        (@token $t:tt $sub:expr) => {$sub};
        (@count $($t:tt)*) => {<[()]>::len(&[$(check_tester!(@token $t ())),*])};
        ($checker:ident;
            $(
                $test:ident => {$(
                    $input:expr => $expected:expr,
                )*}
            ),* $(,)?) => {
            $(
                #[test]
                fn $test() {
                    let mut checker = $checker::default();
                    let total = check_tester!(@count $($expected)*);
                    let mut _line_number = 1;
                    $(
                        let line = line_entry(_line_number, total, $input);
                        _line_number += 1;
                        let result = checker.run(&line);
                        let expected = ($expected).map(|e: &str| Warning::new(line, checker.name(), e));
                        assert_eq!(expected, result);
                    )*
                }
            )*
        };
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
}
