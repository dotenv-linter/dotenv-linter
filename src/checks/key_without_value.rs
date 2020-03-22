use crate::checks::Check;
use crate::common::*;

pub(crate) struct KeyWithoutValueChecker {
    template: String,
}

impl Default for KeyWithoutValueChecker {
    fn default() -> Self {
        Self {
            template: String::from("The {} key should be with a value or have an equal sign"),
        }
    }
}

impl Check for KeyWithoutValueChecker {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if !line.raw_string.contains('=') {
            Some(Warning::new(
                line.clone(),
                self.template.replace("{}", &line.raw_string),
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn working_run_with_value() {
        let mut checker = KeyWithoutValueChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO=BAR"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_run_without_value() {
        let mut checker = KeyWithoutValueChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO="),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn failing_run() {
        let mut checker = KeyWithoutValueChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            String::from("The FOO key should be with a value or have an equal sign"),
        ));
        assert_eq!(expected, checker.run(&line));
    }
}
