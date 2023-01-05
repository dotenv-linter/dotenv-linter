use super::Check;
use crate::common::{comment::Comment, LintKind, Warning};
use dotenv_lookup::LineEntry;

pub(crate) struct UnorderedKeyChecker<'a> {
    template: &'a str,
    keys: Vec<String>,
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
        if line.is_empty()
            || line.get_comment().and_then(Comment::parse).is_some()
            || has_substitution_in_group
        {
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
            line.number,
            self.name(),
            self.message(key, another_key),
        ))
    }

    fn name(&self) -> LintKind {
        LintKind::UnorderedKey
    }

    fn skip_comments(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::check_test;

    #[test]
    fn one_key_test() {
        check_test(&mut UnorderedKeyChecker::default(), [("", None)]);
    }

    #[test]
    fn two_ordered_key_test() {
        check_test(
            &mut UnorderedKeyChecker::default(),
            [("", None), ("", None)],
        );
    }

    #[test]
    fn one_unordered_key_test() {
        check_test(
            &mut UnorderedKeyChecker::default(),
            [
                ("FOO=BAR", None),
                ("BAR=FOO", Some("The BAR key should go before the FOO key")),
            ],
        );
    }

    #[test]
    fn two_unordered_keys_before_test() {
        check_test(
            &mut UnorderedKeyChecker::default(),
            [
                ("FOO=BAR", None),
                ("BAR=FOO", Some("The BAR key should go before the FOO key")),
                ("ABC=BAR", Some("The ABC key should go before the BAR key")),
            ],
        );
    }

    #[test]
    fn two_unordered_keys_before_and_after_test() {
        check_test(
            &mut UnorderedKeyChecker::default(),
            [
                ("FOO=BAR", None),
                ("BAR=FOO", Some("The BAR key should go before the FOO key")),
                ("DDD=BAR", Some("The DDD key should go before the FOO key")),
            ],
        );
    }

    #[test]
    fn two_ordered_and_two_unordered_keys_test() {
        check_test(
            &mut UnorderedKeyChecker::default(),
            [
                ("FOO=BAR", None),
                ("BAR=FOO", Some("The BAR key should go before the FOO key")),
                ("DDD=BAR", Some("The DDD key should go before the FOO key")),
                ("ZOO=BAR", None),
            ],
        );
    }

    #[test]
    fn one_unordered_key_with_blank_line_test() {
        check_test(
            &mut UnorderedKeyChecker::default(),
            [("FOO=BAR", None), ("", None), ("BAR=FOO", None)],
        );
    }

    #[test]
    fn one_unordered_key_with_control_comment_test() {
        check_test(
            &mut UnorderedKeyChecker::default(),
            [
                ("FOO=BAR", None),
                ("# dotenv-linter:off LowercaseKey", None),
                ("Bar=FOO", None),
            ],
        );
    }

    #[test]
    fn two_ordered_groups_with_two_substitution_keys_test() {
        check_test(
            &mut UnorderedKeyChecker::default(),
            [
                ("ABC=XYZ", None),
                ("KEY=VALUE", None),
                ("FOO=$KEY # Unordered, FOO uses KEY", None),
                ("BAR=FOO", None),
                ("BOO=$FOO", None),
                ("XYZ=ABC", None),
            ],
        );
    }

    #[test]
    fn three_ordered_groups_with_two_unordered_substitution_keys_that_have_multiple_values_test() {
        check_test(
            &mut UnorderedKeyChecker::default(),
            [
                ("BBB=XYZ", None),
                ("HHH=VAL", None),
                ("ZZZ=VALUE", None),
                ("CCC=$NNN$HHH # Unordered, CCC uses HHH", None),
                ("BAR=FOOD", None),
                ("BIG=FOOT", None),
                ("WWW=$XYZ$TTT", None),
                ("YYY=FOO", None),
                ("BOO=$BAR$BBB$ZZZ # Unordered, BOO uses BAR", None),
                ("TTT=BIG", None),
                ("XYZ=G", None),
            ],
        );
    }

    #[test]
    fn two_unordered_groups_before_and_after_unordered_substitution_keys_test() {
        check_test(
            &mut UnorderedKeyChecker::default(),
            [
                ("HHH=VAL", None),
                ("ZZZ=VALUE", None),
                ("BBB=$XYZ", Some("The BBB key should go before the HHH key")),
                ("CCC=$HHH # Unordered, CCC uses HHH", None),
                ("TTT=BIG", None),
                ("XYZ=G", None),
                ("GGG=JJJ", Some("The GGG key should go before the TTT key")),
                ("AAA=$ZZZ", Some("The AAA key should go before the GGG key")),
                ("MMM=$GGG$ZZZ # Unordered, MMM uses GGG", None),
            ],
        );
    }
}
