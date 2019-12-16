use crate::checks::{Check, Warning};
use crate::LineEntry;

pub(crate) struct KeyWithoutValueChecker {
    warning: Warning,
}

impl Default for KeyWithoutValueChecker {
    fn default() -> Self {
        Self {
            warning: Warning::new("The {} key should be with a value or have an equal sign"),
        }
    }
}

impl Check for KeyWithoutValueChecker {
    fn run(&self, line: &LineEntry) -> Option<Warning> {
        if !line.raw_string.contains('=') {
            Some(Warning {
                message: self.warning.message.replace("{}", &line.raw_string),
            })
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
        let expected = Some(Warning::new(
            "The RAILS_ENV key should be with a value or have an equal sign",
        ));
        assert_eq!(expected, checker.run(line));

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
