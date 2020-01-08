use crate::checks::{Check, Warning};
use crate::LineEntry;

pub(crate) struct SpacesAroundEqualChecker {
    template: String,
}

impl Default for SpacesAroundEqualChecker {
    fn default() -> Self {
        Self {
            template: String::from("The line has spaces around equal sign"),
        }
    }
}

impl Check for SpacesAroundEqualChecker {
    fn run(&mut self, line: LineEntry) -> Option<Warning> {
        let line_splitted = line.raw_string.split('=').collect::<Vec<&str>>();

        if let [key, value] = &line_splitted[..] {
            if key.ends_with(' ') || value.starts_with(' ') {
                return Some(Warning::new(line, self.template.clone()));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGE: &str = "The line has spaces around equal sign";

    #[test]
    fn working_run() {
        let mut checker = SpacesAroundEqualChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn working_leading_run() {
        let mut checker = SpacesAroundEqualChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from(" DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn working_trailing_run() {
        let mut checker = SpacesAroundEqualChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("DEBUG_HTTP=true "),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn working_empty_run() {
        let mut checker = SpacesAroundEqualChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from(""),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn working_no_equal_sign_run() {
        let mut checker = SpacesAroundEqualChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("DEBUG_HTTP true"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn failing_run() {
        let mut checker = SpacesAroundEqualChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("DEBUG-HTTP = true"),
        };
        let expected = Some(Warning::new(line, MESSAGE.to_string()));
        assert_eq!(expected, checker.run(line));
    }

    #[test]
    fn failing_when_whitespace_before_equal_sign_run() {
        let mut checker = SpacesAroundEqualChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("DEBUG-HTTP =true"),
        };
        let expected = Some(Warning::new(line, MESSAGE.to_string()));
        assert_eq!(expected, checker.run(line));
    }

    #[test]
    fn failing_when_whitespace_after_equal_sign_run() {
        let mut checker = SpacesAroundEqualChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("DEBUG-HTTP= true"),
        };
        let expected = Some(Warning::new(line, MESSAGE.to_string()));
        assert_eq!(expected, checker.run(line));
    }
}
