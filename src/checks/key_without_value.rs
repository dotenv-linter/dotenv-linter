use crate::checks::Check;
use crate::common::*;

pub(crate) struct KeyWithoutValueChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl Default for KeyWithoutValueChecker<'_> {
    fn default() -> Self {
        Self {
            name: "KeyWithoutValue",
            template: "The {} key should be with a value or have an equal sign",
        }
    }
}

impl KeyWithoutValueChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", &key)
    }
}

impl Check for KeyWithoutValueChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if !(line.is_empty() || line.raw_string.contains('=')) {
            Some(Warning::new(
                line.clone(),
                self.name(),
                self.message(&line.raw_string),
            ))
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        self.name
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
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("FOO=BAR"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_run_with_blank_line() {
        let mut checker = KeyWithoutValueChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from(""),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_run_without_value() {
        let mut checker = KeyWithoutValueChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("FOO="),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn failing_run() {
        let mut checker = KeyWithoutValueChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("FOO"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            "KeyWithoutValue",
            String::from("The FOO key should be with a value or have an equal sign"),
        ));
        assert_eq!(expected, checker.run(&line));
    }
}
