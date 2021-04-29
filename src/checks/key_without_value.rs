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
                self.message(line.get_key().unwrap_or(&line.raw_string)),
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
    use crate::common::tests::*;

    #[test]
    fn working_run_with_value() {
        check_test(&mut KeyWithoutValueChecker::default(), [("FOO=BAR", None)]);
    }

    #[test]
    fn working_run_with_blank_line() {
        check_test(&mut KeyWithoutValueChecker::default(), [("", None)]);
    }

    #[test]
    fn working_run_without_value() {
        check_test(&mut KeyWithoutValueChecker::default(), [("FOO=", None)]);
    }

    #[test]
    fn failing_run() {
        check_test(
            &mut KeyWithoutValueChecker::default(),
            [(
                "FOO",
                Some("The FOO key should be with a value or have an equal sign"),
            )],
        );
    }
}
