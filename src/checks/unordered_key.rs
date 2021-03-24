use crate::checks::Check;
use crate::common::*;

pub(crate) struct UnorderedKeyChecker<'a> {
    template: &'a str,
    keys: Vec<String>,
    name: &'a str,
}

impl UnorderedKeyChecker<'_> {
    fn message(&self, key_one: &str, key_two: &str) -> String {
        self.template
            .replace("{1}", key_one)
            .replace("{2}", key_two)
    }
}

impl Default for UnorderedKeyChecker<'_> {
    fn default() -> Self {
        Self {
            name: "UnorderedKey",
            keys: Vec::new(),
            template: "The {1} key should go before the {2} key",
        }
    }
}

impl Check for UnorderedKeyChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let has_substitution_in_group = line
            .get_substitution_keys()
            .iter()
            .any(|k| self.keys.iter().any(|key| key == k));

        // Support of grouping variables through blank lines and control comments
        if line.is_empty() || line.get_control_comment().is_some() || has_substitution_in_group {
            self.keys.clear();
            return None;
        }

        let key = line.get_key()?;
        self.keys.push(key.to_string());

        let mut sorted_keys = self.keys.clone();
        sorted_keys.sort();

        if sorted_keys.eq(&self.keys) {
            return None;
        }

        let another_key = sorted_keys.iter().skip_while(|&s| s != key).nth(1)?;

        Some(Warning::new(
            line.clone(),
            self.name(),
            self.message(&key, &another_key),
        ))
    }

    fn name(&self) -> &str {
        self.name
    }

    fn skip_comments(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    fn run_unordered_tests(asserts: Vec<(LineEntry, Option<Warning>)>) {
        let mut checker = UnorderedKeyChecker::default();

        for assert in asserts {
            let (input, output) = assert;
            assert_eq!(checker.run(&input), output);
        }
    }

    #[test]
    fn one_key_test() {
        let asserts = vec![(line_entry(1, 1, ""), None)];

        run_unordered_tests(asserts);
    }

    #[test]
    fn two_ordered_keys_test() {
        let asserts = vec![(line_entry(1, 2, ""), None), (line_entry(2, 2, ""), None)];

        run_unordered_tests(asserts);
    }

    #[test]
    fn one_unordered_key_test() {
        let asserts = vec![
            (line_entry(1, 2, "FOO=BAR"), None),
            (
                line_entry(2, 2, "BAR=FOO"),
                Some(Warning::new(
                    line_entry(2, 2, "BAR=FOO"),
                    "UnorderedKey",
                    "The BAR key should go before the FOO key",
                )),
            ),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn two_unordered_keys_before_test() {
        let asserts = vec![
            (line_entry(1, 3, "FOO=BAR"), None),
            (
                line_entry(2, 3, "BAR=FOO"),
                Some(Warning::new(
                    line_entry(2, 3, "BAR=FOO"),
                    "UnorderedKey",
                    "The BAR key should go before the FOO key",
                )),
            ),
            (
                line_entry(3, 3, "ABC=BAR"),
                Some(Warning::new(
                    line_entry(3, 3, "ABC=BAR"),
                    "UnorderedKey",
                    "The ABC key should go before the BAR key",
                )),
            ),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn two_unordered_keys_before_and_after_test() {
        let asserts = vec![
            (line_entry(1, 3, "FOO=BAR"), None),
            (
                line_entry(2, 3, "BAR=FOO"),
                Some(Warning::new(
                    line_entry(2, 3, "BAR=FOO"),
                    "UnorderedKey",
                    "The BAR key should go before the FOO key",
                )),
            ),
            (
                line_entry(3, 3, "DDD=BAR"),
                Some(Warning::new(
                    line_entry(3, 3, "DDD=BAR"),
                    "UnorderedKey",
                    "The DDD key should go before the FOO key",
                )),
            ),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn two_ordered_and_two_unordered_keys_test() {
        let asserts = vec![
            (line_entry(1, 4, "FOO=BAR"), None),
            (
                line_entry(2, 4, "BAR=FOO"),
                Some(Warning::new(
                    line_entry(2, 4, "BAR=FOO"),
                    "UnorderedKey",
                    "The BAR key should go before the FOO key",
                )),
            ),
            (
                line_entry(3, 4, "DDD=BAR"),
                Some(Warning::new(
                    line_entry(3, 4, "DDD=BAR"),
                    "UnorderedKey",
                    "The DDD key should go before the FOO key",
                )),
            ),
            (line_entry(4, 4, "ZOO=BAR"), None),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn one_unordered_key_with_blank_line_test() {
        let asserts = vec![
            (line_entry(1, 3, "FOO=BAR"), None),
            (line_entry(2, 3, ""), None),
            (line_entry(3, 3, "BAR=FOO"), None),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn one_unordered_key_with_control_comment_test() {
        let asserts = vec![
            (line_entry(1, 3, "FOO=BAR"), None),
            (line_entry(2, 3, "# dotenv-linter:off LowercaseKey"), None),
            (line_entry(3, 3, "Bar=FOO"), None),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn two_ordered_groups_with_two_substitution_keys_test() {
        let asserts = vec![
            (line_entry(1, 3, "ABC=XYZ"), None),
            (line_entry(2, 3, "KEY=VALUE"), None),
            (line_entry(3, 3, "FOO=$KEY # Unordered, FOO uses KEY"), None),
            (line_entry(4, 3, "BAR=FOO"), None),
            (line_entry(5, 3, "BOO=$FOO"), None),
            (line_entry(6, 3, "XYZ=ABC"), None),
        ];

        run_unordered_tests(asserts);
    }
}
