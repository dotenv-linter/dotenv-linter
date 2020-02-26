use crate::checks::Check;
use crate::common::*;

pub(crate) struct LowercaseKeyChecker {
    template: String,
}

impl Default for LowercaseKeyChecker {
    fn default() -> Self {
        Self {
            template: String::from("The {} key should be in uppercase"),
        }
    }
}

impl Check for LowercaseKeyChecker {
    fn run(&mut self, line: LineEntry) -> Option<Warning> {
        let key = line.get_key()?;
        if key.to_uppercase() == key {
            None
        } else {
            Some(Warning::new(line, self.template.replace("{}", &key)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_run() {
        let mut checker = LowercaseKeyChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOO=BAR"),
        };
        assert_eq!(None, checker.run(line));
    }

    #[test]
    fn failing_run_with_lowercase_key() {
        let mut checker = LowercaseKeyChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("foo_bar=FOOBAR"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            String::from("The foo_bar key should be in uppercase"),
        ));
        assert_eq!(expected, checker.run(line));
    }

    #[test]
    fn failing_run_with_lowercase_letter() {
        let mut checker = LowercaseKeyChecker::default();
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOo_BAR=FOOBAR"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            String::from("The FOo_BAR key should be in uppercase"),
        ));
        assert_eq!(expected, checker.run(line));
    }
}
