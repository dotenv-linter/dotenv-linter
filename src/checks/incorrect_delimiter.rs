use crate::checks::{Check, Warning};
use crate::LineEntry;

pub(crate) struct IncorrectDelimiterChecker {
    template: String,
}

impl Default for IncorrectDelimiterChecker {
    fn default() -> Self {
        Self {
            template: String::from("The {} key has incorrect delimiter"),
        }
    }
}

impl Check for IncorrectDelimiterChecker {
    fn run(&mut self, line: LineEntry) -> Option<Warning> {
        let eq_index = line.raw_string.find('=')?;
        let key = line.raw_string.get(0..eq_index)?;
        if key.trim().chars().any(|c| !c.is_alphabetic() && c != '_') {
            return Some(Warning::new(line.clone(), self.template.replace("{}", key)));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOO_BAR=FOOBAR"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn failing_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOO-BAR=FOOBAR"),
        };
        let expected = Some(Warning::new(
            line,
            String::from("The FOO-BAR key has incorrect delimiter"),
        ));
        assert_eq!(expected, checker.run(line));
    }

    #[test]
    fn failing_with_whitespace_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOO BAR=FOOBAR"),
        };
        let expected = Some(Warning::new(
            line,
            String::from("The FOO BAR key has incorrect delimiter"),
        ));
        assert_eq!(expected, checker.run(line));
    }

    #[test]
    fn unformatted_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOO-BAR"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn leading_space_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from(" FOO=FOOBAR"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn trailing_space_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOO_BAR =FOOBAR"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn empty_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from(""),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn short_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("F=BAR"),
        };
        assert_eq!(None, checker.run(line));
    }
}
