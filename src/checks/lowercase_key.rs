use crate::checks::Check;
use crate::common::*;

pub(crate) struct LowercaseKeyChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl Default for LowercaseKeyChecker<'_> {
    fn default() -> Self {
        Self {
            name: "LowercaseKey",
            template: "The {} key should be in uppercase",
        }
    }
}

impl Check for LowercaseKeyChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let key = line.get_key()?;
        if key.to_uppercase() == key {
            None
        } else {
            Some(Warning::new(line.clone(), self.message(&key)))
        }
    }
}

impl LowercaseKeyChecker<'_> {
    fn message(&self, key: &str) -> String {
        format!("{}: {}", self.name, self.template.replace("{}", key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn working_run() {
        let mut checker = LowercaseKeyChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO=BAR"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn failing_run_with_lowercase_key() {
        let mut checker = LowercaseKeyChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("foo_bar=FOOBAR"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            String::from("LowercaseKey: The foo_bar key should be in uppercase"),
        ));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn failing_run_with_lowercase_letter() {
        let mut checker = LowercaseKeyChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOo_BAR=FOOBAR"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            String::from("LowercaseKey: The FOo_BAR key should be in uppercase"),
        ));
        assert_eq!(expected, checker.run(&line));
    }
}
