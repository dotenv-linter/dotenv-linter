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
    use crate::{check_tester, common::tests::*};

    check_tester!{
        KeyWithoutValueChecker;
        working_run_with_value => {
            "FOO=BAR" => None,
        },
        working_run_with_blank_line => {
            "" => None,
        },
        working_run_without_value => {
            "FOO=" => None,
        },
        failing_run => {
            "FOO" => Some("The FOO key should be with a value or have an equal sign"),
        }
    }

}
