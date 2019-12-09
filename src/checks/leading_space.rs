use crate::checks::{Check, Warning};
use crate::LineEntry;

pub(crate) struct LeadingSpaceChecker {
    warning: Warning,
}

impl Default for LeadingSpaceChecker {
    fn default() -> Self {
        Self {
            warning: Warning::new("Leading space detected"),
        }
    }
}

impl Check for LeadingSpaceChecker {
    fn run(&self, line: &LineEntry) -> Option<Warning> {
        if line.raw_string.starts_with(' ') {
            Some(self.warning.clone())
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

        let line = &LineEntry {
            number: 1,
            raw_string: String::from(" DEBUG_HTTP=true"),
        };
        assert_eq!(Some(checker.warning.to_owned()), checker.run(line));

        let line = &LineEntry {
            number: 1,
            raw_string: String::from("  DEBUG_HTTP=true"),
        };
        assert_eq!(Some(checker.warning.to_owned()), checker.run(line));

        let line = &LineEntry {
            number: 1,
            raw_string: String::from("    DEBUG_HTTP=true"),
        };
        assert_eq!(Some(checker.warning.to_owned()), checker.run(line));
    }
}
