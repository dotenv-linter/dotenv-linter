use crate::checks::Check;
use crate::common::*;

pub(crate) struct LeadingSpaceChecker {
    template: String,
}

impl Default for LeadingSpaceChecker {
    fn default() -> Self {
        Self {
            template: String::from("Leading space detected"),
        }
    }
}

impl Check for LeadingSpaceChecker {
    fn run(&mut self, line: LineEntry) -> Option<Warning> {
        if line
            .raw_string
            .starts_with(|c: char| c.is_alphabetic() || c == '_')
        {
            None
        } else {
            Some(Warning::new(line, self.template.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGE: &str = "Leading space detected";

    #[test]
    fn normal() {
        let mut checker = LeadingSpaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOO=BAR"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn leading_underscore() {
        let mut checker = LeadingSpaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("_FOO=BAR"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn leading_dot() {
        let mut checker = LeadingSpaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from(".FOO=BAR"),
        };
        assert_eq!(
            Some(Warning::new(line.clone(), MESSAGE.to_string())),
            checker.run(line)
        );
    }

    #[test]
    fn leading_asterisk() {
        let mut checker = LeadingSpaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("*FOO=BAR"),
        };
        assert_eq!(
            Some(Warning::new(line.clone(), MESSAGE.to_string())),
            checker.run(line)
        );
    }

    #[test]
    fn leading_number() {
        let mut checker = LeadingSpaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("1FOO=BAR"),
        };
        assert_eq!(
            Some(Warning::new(line.clone(), MESSAGE.to_string())),
            checker.run(line)
        );
    }

    #[test]
    fn leading_space() {
        let mut checker = LeadingSpaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from(" FOO=BAR"),
        };
        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(line));
    }

    #[test]
    fn two_leading_spaces() {
        let mut checker = LeadingSpaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("  FOO=BAR"),
        };
        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(line));
    }

    #[test]
    fn leading_tab() {
        let mut checker = LeadingSpaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("\tFOO=BAR"),
        };
        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(line));
    }
}
