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
        // FIXME: Doesn't check a tab character
        if line.raw_string.starts_with(' ') {
            Some(Warning::new(line, self.template.clone()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGE: &str = "Leading space detected";

    #[test]
    fn working_run() {
        let mut checker = LeadingSpaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOO=BAR"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn failing_run_with_one_leading_space() {
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
    fn failing_run_with_two_leading_spaces() {
        let mut checker = LeadingSpaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("  FOO=BAR"),
        };
        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(line));
    }
}
