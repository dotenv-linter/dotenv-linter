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
            Some(Warning::new(line.clone(), self.name(), self.message(&key)))
        }
    }

    fn name(&self) -> &str {
        self.name
    }
}

impl LowercaseKeyChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn working_run() {
        let mut checker = LowercaseKeyChecker::default();
        let line = line_entry(1, 1, "FOO=BAR");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn failing_run_with_lowercase_key() {
        let mut checker = LowercaseKeyChecker::default();
        let line = line_entry(1, 1, "foo_bar=FOOBAR");
        let expected = Some(Warning::new(
            line.clone(),
            "LowercaseKey",
            String::from("The foo_bar key should be in uppercase"),
        ));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn failing_run_with_lowercase_letter() {
        let mut checker = LowercaseKeyChecker::default();
        let line = line_entry(1, 1, "FOo_BAR=FOOBAR");
        let expected = Some(Warning::new(
            line.clone(),
            "LowercaseKey",
            String::from("The FOo_BAR key should be in uppercase"),
        ));
        assert_eq!(expected, checker.run(&line));
    }
}
