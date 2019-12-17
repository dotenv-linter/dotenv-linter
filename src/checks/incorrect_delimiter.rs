use crate::checks::{Check, Warning};
use crate::LineEntry;

pub(crate) struct IncorrectDelimiterChecker {
    warning: Warning,
}

impl Default for IncorrectDelimiterChecker {
    fn default() -> Self {
        Self {
            warning: Warning::new("The {} key has incorrect delimiter"),
        }
    }
}

impl Check for IncorrectDelimiterChecker {
    fn run(&self, line: &LineEntry) -> Option<Warning> {
        let eq_index = line.raw_string.find('=')?;
        let key = line.raw_string.get(0..eq_index)?;
        if key.trim().chars().any(|c| !c.is_alphabetic() && c != '_') {
            return Some(Warning {
                message: self.warning.message.replace("{}", key),
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_run() {
        let checker = IncorrectDelimiterChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn failing_run() {
        let checker = IncorrectDelimiterChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("DEBUG-HTTP=true"),
        };
        let expected = Some(Warning::new("The DEBUG-HTTP key has incorrect delimiter"));
        assert_eq!(expected, checker.run(line));
    }

    #[test]
    fn failing_with_whitepsace_run() {
        let checker = IncorrectDelimiterChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("DEBUG HTTP=true"),
        };
        let expected = Some(Warning::new("The DEBUG HTTP key has incorrect delimiter"));
        assert_eq!(expected, checker.run(line));
    }

    #[test]
    fn unformated_run() {
        let checker = IncorrectDelimiterChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("DEBUG-HTTPtrue"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn leading_space_run() {
        let checker = IncorrectDelimiterChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from(" DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn trailing_space_run() {
        let checker = IncorrectDelimiterChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("DEBUG_HTTP =true"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn empty_run() {
        let checker = IncorrectDelimiterChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from(""),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn short_run() {
        let checker = IncorrectDelimiterChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("A=short"),
        };
        assert_eq!(None, checker.run(line));
    }
}
