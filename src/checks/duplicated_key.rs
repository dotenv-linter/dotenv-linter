use crate::checks::Check;
use crate::common::*;
use std::collections::HashSet;

pub(crate) struct DuplicatedKeyChecker<'a> {
    name: &'a str,
    template: &'a str,
    keys: HashSet<String>,
}

impl DuplicatedKeyChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", &key)
    }
}

impl Default for DuplicatedKeyChecker<'_> {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
            name: "DuplicatedKey",
            template: "The {} key is duplicated",
        }
    }
}

impl Check for DuplicatedKeyChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let key = line.get_key()?;

        if self.keys.contains(key) {
            return Some(Warning::new(line.clone(), self.name(), self.message(&key)));
        }

        self.keys.insert(key.to_string());
        None
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
        DuplicatedKeyChecker;
        with_one_duplicated_key_test => {
            "FOO=BAR" => None,
            "FOO=BAR" => Some("The FOO key is duplicated"),
        },
        with_two_unique_keys_test => {
            "FOO=BAR" => None,
            "BAR=FOO" => None,
        },
        with_two_unique_keys_case_sensitive_test => {
            "FOO=BAR" => None,
            "Foo=FOO" => None,
        },
        with_two_duplicated_keys_test => {
            "FOO=BAR" => None,
            "FOO=BAR" => Some("The FOO key is duplicated"),
            "BAR=FOO" => None,
            "BAR=FOO" => Some("The BAR key is duplicated"),
        },
        one_duplicated_and_one_unique_key_test => {
            "FOO=BAR" => None,
            "FOO=BAR" => Some("The FOO key is duplicated"),
            "BAR=FOO" => None,
        }
    }
}
