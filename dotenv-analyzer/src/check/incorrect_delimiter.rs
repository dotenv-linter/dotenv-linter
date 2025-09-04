use dotenv_core::LineEntry;

use super::Check;
use crate::{remove_invalid_leading_chars, LintKind, Warning};

pub(crate) struct IncorrectDelimiterChecker<'a> {
    template: &'a str,
}

impl IncorrectDelimiterChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", key)
    }
}

impl Default for IncorrectDelimiterChecker<'_> {
    fn default() -> Self {
        Self {
            template: "The {} key has incorrect delimiter",
        }
    }
}

impl Check for IncorrectDelimiterChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let key = line.get_key()?;

        // delimiters occur /between/ characters, not as the initial character, so we should
        // remove all invalid leading characters before checking for incorrect delimiters
        let cleaned_key = remove_invalid_leading_chars(key);

        if cleaned_key
            .trim()
            .chars()
            .any(|c| !c.is_alphanumeric() && c != '_')
        {
            return Some(Warning::new(line.number, self.name(), self.message(key)));
        }

        None
    }

    fn name(&self) -> LintKind {
        LintKind::IncorrectDelimiter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::check_test;

    #[test]
    fn working_run() {
        check_test(
            &mut IncorrectDelimiterChecker::default(),
            [("FOO_BAR=FOOBAR", None)],
        );
    }

    #[test]
    fn working_with_digits_run() {
        check_test(
            &mut IncorrectDelimiterChecker::default(),
            [("F100=BAR", None)],
        );
    }

    #[test]
    fn working_with_export_run() {
        check_test(
            &mut IncorrectDelimiterChecker::default(),
            [("export FOO=BAR", None)],
        );
    }

    #[test]
    fn incorrect_leading_char() {
        check_test(
            &mut IncorrectDelimiterChecker::default(),
            [("*FOO=BAR", None)],
        );
    }

    #[test]
    fn incorrect_leading_chars_and_invalid_delimiter() {
        check_test(
            &mut IncorrectDelimiterChecker::default(),
            [(
                "***F-OOBAR=BAZ",
                Some("The ***F-OOBAR key has incorrect delimiter"),
            )],
        );
    }

    #[test]
    fn incorrect_ending_delimiter() {
        check_test(
            &mut IncorrectDelimiterChecker::default(),
            [("FOO*=BAR", Some("The FOO* key has incorrect delimiter"))],
        );
    }

    #[test]
    fn failing_run() {
        check_test(
            &mut IncorrectDelimiterChecker::default(),
            [(
                "FOO-BAR=FOOBAR",
                Some("The FOO-BAR key has incorrect delimiter"),
            )],
        );
    }

    #[test]
    fn failing_with_whitespace_run() {
        check_test(
            &mut IncorrectDelimiterChecker::default(),
            [(
                "FOO BAR=FOOBAR",
                Some("The FOO BAR key has incorrect delimiter"),
            )],
        );
    }

    #[test]
    fn unformatted_run() {
        check_test(
            &mut IncorrectDelimiterChecker::default(),
            [("FOO-BAR", Some("The FOO-BAR key has incorrect delimiter"))],
        );
    }

    #[test]
    fn trailing_space_run() {
        check_test(
            &mut IncorrectDelimiterChecker::default(),
            [("FOO_BAR =FOOBAR", None)],
        );
    }

    #[test]
    fn empty_run() {
        check_test(&mut IncorrectDelimiterChecker::default(), [("", None)]);
    }

    #[test]
    fn short_run() {
        check_test(&mut IncorrectDelimiterChecker::default(), [("F=BAR", None)]);
    }
}
