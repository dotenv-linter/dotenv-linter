use crate::checks::Check;
use crate::common::*;

pub(crate) struct LeadingCharacterChecker {
    name: &'static str,
    template: &'static str,
}

impl Default for LeadingCharacterChecker {
    fn default() -> Self {
        Self {
            name: "LeadingCharacter",
            template: "Invalid leading character detected",
        }
    }
}

impl LeadingCharacterChecker {
    fn message(&self) -> String {
        format!("{}: {}", self.name, self.template)
    }
}

impl Check for LeadingCharacterChecker {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if line.is_empty()
            || line
                .raw_string
                .starts_with(|c: char| c.is_alphabetic() || c == '_')
        {
            None
        } else {
            Some(Warning::new(line.clone(), self.message()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const MESSAGE: &str = "LeadingCharacter: Invalid leading character detected";

    #[test]
    fn normal() {
        let mut checker = LeadingCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO=BAR"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn blank_line() {
        let mut checker = LeadingCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from(""),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn leading_underscore() {
        let mut checker = LeadingCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("_FOO=BAR"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn leading_dot() {
        let mut checker = LeadingCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from(".FOO=BAR"),
        };
        assert_eq!(
            Some(Warning::new(line.clone(), MESSAGE.to_string())),
            checker.run(&line)
        );
    }

    #[test]
    fn leading_asterisk() {
        let mut checker = LeadingCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("*FOO=BAR"),
        };
        assert_eq!(
            Some(Warning::new(line.clone(), MESSAGE.to_string())),
            checker.run(&line)
        );
    }

    #[test]
    fn leading_number() {
        let mut checker = LeadingCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("1FOO=BAR"),
        };
        assert_eq!(
            Some(Warning::new(line.clone(), MESSAGE.to_string())),
            checker.run(&line)
        );
    }

    #[test]
    fn leading_space() {
        let mut checker = LeadingCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from(" FOO=BAR"),
        };
        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn two_leading_spaces() {
        let mut checker = LeadingCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("  FOO=BAR"),
        };
        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn leading_tab() {
        let mut checker = LeadingCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("\tFOO=BAR"),
        };
        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(&line));
    }
}
