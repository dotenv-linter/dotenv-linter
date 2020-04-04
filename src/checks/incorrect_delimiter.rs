use crate::checks::Check;
use crate::common::*;

pub(crate) struct IncorrectDelimiterChecker {
    name: &'static str,
    template: &'static str,
}

impl IncorrectDelimiterChecker {
    fn message(&self, key: &str) -> String {
        format!("{}: {}", self.name, self.template.replace("{}", &key))
    }
}

impl Default for IncorrectDelimiterChecker {
    fn default() -> Self {
        Self {
            name: "IncorrectDelimiter",
            template: "The {} key has incorrect delimiter",
        }
    }
}

impl Check for IncorrectDelimiterChecker {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let key = line.get_key()?;
        if key.trim().chars().any(|c| !c.is_alphanumeric() && c != '_') {
            return Some(Warning::new(line.clone(), self.message(&key)));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn working_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO_BAR=FOOBAR"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_with_digits_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("F1OO=BAR"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn failing_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO-BAR=FOOBAR"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            String::from("IncorrectDelimiter: The FOO-BAR key has incorrect delimiter"),
        ));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn failing_with_whitespace_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO BAR=FOOBAR"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            String::from("IncorrectDelimiter: The FOO BAR key has incorrect delimiter"),
        ));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn unformatted_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO-BAR"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn leading_space_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from(" FOO=FOOBAR"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn trailing_space_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO_BAR =FOOBAR"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn empty_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from(""),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn short_run() {
        let mut checker = IncorrectDelimiterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("F=BAR"),
        };
        assert_eq!(None, checker.run(&line));
    }
}
