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
    fn run<'l>(&mut self, line: &'l LineEntry) -> Option<Warning<'l>> {
        let key = line.get_key()?;

        if self.keys.contains(key) {
            return Some(Warning::new(line, self.name(), self.message(&key)));
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
    use crate::common::tests::*;

    fn run_duplicated_tests(asserts: Vec<(LineEntry, Option<Warning>)>) {
        let mut checker = DuplicatedKeyChecker::default();

        for assert in asserts {
            let (input, output) = assert;
            assert_eq!(checker.run(&input), output);
        }
    }

    #[test]
    fn with_one_duplicated_key_test() {
        let asserts = vec![
            (line_entry(1, 2, "FOO=BAR"), None),
            (
                line_entry(2, 2, "FOO=BAR"),
                Some(Warning::new(
                    line_entry(2, 2, "FOO=BAR"),
                    "DuplicatedKey",
                    "The FOO key is duplicated",
                )),
            ),
        ];

        run_duplicated_tests(asserts);
    }

    #[test]
    fn with_two_unique_keys_test() {
        let asserts = vec![
            (line_entry(1, 2, "FOO=BAR"), None),
            (line_entry(2, 2, "BAR=FOO"), None),
        ];

        run_duplicated_tests(asserts);
    }
    #[test]
    fn with_two_unique_keys_case_sensitive_test() {
        let asserts = vec![
            (line_entry(1, 2, "FOO=BAR"), None),
            (line_entry(2, 2, "Foo=FOO"), None),
        ];

        run_duplicated_tests(asserts);
    }

    #[test]
    fn with_two_duplicated_keys_test() {
        let asserts = vec![
            (line_entry(1, 4, "FOO=BAR"), None),
            (
                line_entry(2, 4, "FOO=BAR"),
                Some(Warning::new(
                    line_entry(2, 4, "FOO=BAR"),
                    "DuplicatedKey",
                    "The FOO key is duplicated",
                )),
            ),
            (line_entry(3, 4, "BAR=FOO"), None),
            (
                line_entry(4, 4, "BAR=FOO"),
                Some(Warning::new(
                    line_entry(4, 4, "BAR=FOO"),
                    "DuplicatedKey",
                    "The BAR key is duplicated",
                )),
            ),
        ];

        run_duplicated_tests(asserts);
    }

    #[test]
    fn one_duplicated_and_one_unique_key_test() {
        let asserts = vec![
            (line_entry(1, 3, "FOO=BAR"), None),
            (
                line_entry(2, 3, "FOO=BAR"),
                Some(Warning::new(
                    line_entry(2, 3, "FOO=BAR"),
                    "DuplicatedKey",
                    "The FOO key is duplicated",
                )),
            ),
            (line_entry(3, 3, "BAR=FOO"), None),
        ];

        run_duplicated_tests(asserts);
    }
}
