use crate::checks::{Check, Warning};
use crate::LineEntry;

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
    fn run(&self, line: &LineEntry) -> Option<Warning> {
        if line.raw_string.starts_with(' ') {
            Some(Warning::new(self.template.clone()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_space_checker_run() {
        let checker = LeadingSpaceChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(line));

        let expected = Some(Warning::from("Leading space detected"));

        let line = &LineEntry {
            number: 1,
            raw_string: String::from(" DEBUG_HTTP=true"),
        };
        assert_eq!(expected, checker.run(line));

        let line = &LineEntry {
            number: 1,
            raw_string: String::from("  DEBUG_HTTP=true"),
        };
        assert_eq!(expected, checker.run(line));

        let line = &LineEntry {
            number: 1,
            raw_string: String::from("    DEBUG_HTTP=true"),
        };
        assert_eq!(expected, checker.run(line));
    }
}
