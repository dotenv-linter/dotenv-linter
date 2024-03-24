use super::Check;
use crate::common::{LintKind, Warning};
use dotenv_lookup::LineEntry;

pub(crate) struct LowercaseKeyChecker<'a> {
    template: &'a str,
}

impl LowercaseKeyChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", key)
    }
}

impl Default for LowercaseKeyChecker<'_> {
    fn default() -> Self {
        Self {
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
            Some(Warning::new(line.number, self.name(), self.message(key)))
        }
    }

    fn name(&self) -> LintKind {
        LintKind::LowercaseKey
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::check_test;

    #[test]
    fn working_run() {
        check_test(&mut LowercaseKeyChecker::default(), [("FOO=BAR", None)]);
    }

    #[test]
    fn working_with_export_run() {
        check_test(
            &mut LowercaseKeyChecker::default(),
            [("export FOO=BAR", None)],
        );
    }

    #[test]
    fn failing_run_with_lowercase_key() {
        check_test(
            &mut LowercaseKeyChecker::default(),
            [(
                "foo_bar=FOOBAR",
                Some("The foo_bar key should be in uppercase"),
            )],
        );
    }

    #[test]
    fn failing_run_with_lowercase_letter() {
        check_test(
            &mut LowercaseKeyChecker::default(),
            [(
                "FOo_BAR=FOOBAR",
                Some("The FOo_BAR key should be in uppercase"),
            )],
        );
    }
}
