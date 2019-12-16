use crate::checks::{Check, Warning};
use crate::LineEntry;

pub(crate) struct KeyWithoutValueChecker {
    warning: Warning,
}

impl Default for KeyWithoutValueChecker {
    fn default() -> Self {
        Self {
            warning: Warning::new("Key without value detected"),
        }
    }
}

impl Check for KeyWithoutValueChecker {
    fn run(&self, line: &LineEntry) -> Option<Warning> {
        if !line.raw_string.contains('=') {
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
    fn key_without_value_checker_run() {
        let checker = KeyWithoutValueChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("RAILS_ENV"),
        };
        assert_eq!(Some(checker.warning.to_owned()), checker.run(line));

        let line = &LineEntry {
            number: 1,
            raw_string: String::from("RAILES_ENV="),
        };
        assert_eq!(None, checker.run(line));

        let line = &LineEntry {
            number: 1,
            raw_string: String::from("RAILES_ENV=development"),
        };
        assert_eq!(None, checker.run(line));
    }
}
