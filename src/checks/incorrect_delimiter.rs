use crate::checks::Check;
use crate::common::*;

pub(crate) struct IncorrectDelimiterChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl IncorrectDelimiterChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", &key)
    }
}

impl Default for IncorrectDelimiterChecker<'_> {
    fn default() -> Self {
        Self {
            name: "IncorrectDelimiter",
            template: "The {} key has incorrect delimiter",
        }
    }
}

impl Check for IncorrectDelimiterChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let key = line.get_key()?;

        // delimiters occur /between/ characters, not as the initial character, so we should
        // remove all invalid leading characters before checking for incorrect delimiters
        let cleaned_key = remove_invalid_leading_chars(&key);

        if cleaned_key
            .trim()
            .chars()
            .any(|c| !c.is_alphanumeric() && c != '_')
        {
            return Some(Warning::new(line.clone(), self.name(), self.message(&key)));
        }

        None
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn working_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "FOO_BAR=FOOBAR");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_with_digits_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "F1OO=BAR");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_with_export_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "export FOO=BAR");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn incorrect_leading_char() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "*FOO=BAR");
        // expect None because this warning should be found by LeadingCharacterChecker
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn incorrect_leading_chars_and_invalid_delimiter() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "***F-OOBAR=BAZ");

        let expected = Some(Warning::new(
            line.clone(),
            "IncorrectDelimiter",
            String::from("The ***F-OOBAR key has incorrect delimiter"),
        ));

        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn incorrect_ending_delimiter() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "FOO*=BAR");

        let expected = Some(Warning::new(
            line.clone(),
            "IncorrectDelimiter",
            String::from("The FOO* key has incorrect delimiter"),
        ));

        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn failing_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "FOO-BAR=FOOBAR");
        let expected = Some(Warning::new(
            line.clone(),
            "IncorrectDelimiter",
            String::from("The FOO-BAR key has incorrect delimiter"),
        ));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn failing_with_whitespace_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "FOO BAR=FOOBAR");
        let expected = Some(Warning::new(
            line.clone(),
            "IncorrectDelimiter",
            String::from("The FOO BAR key has incorrect delimiter"),
        ));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn unformatted_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "FOO-BAR");
        let expected = Some(Warning::new(
            line.clone(),
            "IncorrectDelimiter",
            String::from("The FOO-BAR key has incorrect delimiter"),
        ));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn trailing_space_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "FOO_BAR =FOOBAR");
        // has a trailing space, so SpaceCharacterChecker should catch this error
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn empty_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn short_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = line_entry(1, 1, "F=BAR");
        assert_eq!(None, checker.run(&line));
    }
}
