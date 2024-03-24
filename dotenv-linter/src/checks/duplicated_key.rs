use super::Check;
use crate::common::{LintKind, Warning};
use dotenv_lookup::LineEntry;
use std::collections::HashSet;

pub(crate) struct DuplicatedKeyChecker<'a> {
    template: &'a str,
    keys: HashSet<String>,
}

impl DuplicatedKeyChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", key)
    }
}

impl Default for DuplicatedKeyChecker<'_> {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
            template: "The {} key is duplicated",
        }
    }
}

impl Check for DuplicatedKeyChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let key = line.get_key()?;

        if self.keys.contains(key) {
            return Some(Warning::new(line.number, self.name(), self.message(key)));
        }

        self.keys.insert(key.to_string());
        None
    }

    fn name(&self) -> LintKind {
        LintKind::DuplicatedKey
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::check_test;

    #[test]
    fn with_one_duplicated_key_test() {
        check_test(
            &mut DuplicatedKeyChecker::default(),
            [
                ("FOO=BAR", None),
                ("FOO=BAR", Some("The FOO key is duplicated")),
            ],
        );
    }

    #[test]
    fn with_two_unique_keys_test() {
        check_test(
            &mut DuplicatedKeyChecker::default(),
            [("FOO=BAR", None), ("BAR=FOO", None)],
        );
    }

    #[test]
    fn with_two_unique_keys_case_sensitive_test() {
        check_test(
            &mut DuplicatedKeyChecker::default(),
            [("FOO=BAR", None), ("Foo=FOO", None)],
        );
    }

    #[test]
    fn with_two_duplicated_keys_test() {
        check_test(
            &mut DuplicatedKeyChecker::default(),
            [
                ("FOO=BAR", None),
                ("FOO=BAR", Some("The FOO key is duplicated")),
                ("BAR=FOO", None),
                ("BAR=FOO", Some("The BAR key is duplicated")),
            ],
        );
    }

    #[test]
    fn one_duplicated_and_one_unique_key_test() {
        check_test(
            &mut DuplicatedKeyChecker::default(),
            [
                ("FOO=BAR", None),
                ("FOO=BAR", Some("The FOO key is duplicated")),
                ("BAR=FOO", None),
            ],
        );
    }
}
