use super::Check;
use crate::common::{LineEntry, LintKind, Warning};

pub(crate) struct SubstitutionKeyChecker<'a> {
    template: &'a str,
}

impl SubstitutionKeyChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", key)
    }
}

impl Default for SubstitutionKeyChecker<'_> {
    fn default() -> Self {
        Self {
            template: "The {} key is not assigned properly",
        }
    }
}

impl Check for SubstitutionKeyChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let mut value = match line.get_value().map(str::trim) {
            Some(value) if !value.starts_with('\'') => value,
            _ => return None,
        };

        let is_escaped =
            |prefix: &str| prefix.chars().rev().take_while(|ch| *ch == '\\').count() % 2 == 1;

        // Checks if keys used in value have both '{' '}' or neither
        while let Some(index) = value.find('$') {
            let prefix = &value[..index];
            let raw_key = &value[index + 1..];

            // Separate initial key from the rest
            let (initial_key, rest) = raw_key
                .find('$')
                .map(|i| raw_key.split_at(i))
                .unwrap_or_else(|| (raw_key, ""));

            let end_brace_index = initial_key.find('}');
            let has_start_brace = initial_key.starts_with('{');
            let has_end_brace = end_brace_index.is_some();
            let is_incorrect_substitution = has_start_brace ^ has_end_brace
                || end_brace_index
                    .map(|i| &initial_key[1..i])
                    .filter(|key| key.contains(|c: char| !c.is_ascii_alphanumeric() && c != '_'))
                    .is_some();

            if is_incorrect_substitution && !is_escaped(prefix) {
                return Some(Warning::new(
                    line.number,
                    self.name(),
                    self.message(line.get_key()?),
                ));
            }

            value = rest;
        }
        None
    }

    fn name(&self) -> LintKind {
        LintKind::SubstitutionKey
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::check_test;

    #[test]
    fn correct_substitution_key_test() {
        check_test(
            &mut SubstitutionKeyChecker::default(),
            [
                ("ABC=$BAR", None),
                ("FOO=${BAR}", None),
                ("FOO=\"$BAR\"", None),
            ],
        );
    }

    #[test]
    fn incorrect_substitution_key_test() {
        check_test(
            &mut SubstitutionKeyChecker::default(),
            [
                ("ABC=${BAR", Some("The ABC key is not assigned properly")),
                ("FOO=${BAR!}", Some("The FOO key is not assigned properly")),
                ("XYZ=$BAR}", Some("The XYZ key is not assigned properly")),
            ],
        );
    }

    #[test]
    fn multiple_substitution_key_test() {
        check_test(
            &mut SubstitutionKeyChecker::default(),
            [
                ("ABC=${BAR}$XYZ", None),
                ("FOO=$ABC{${BAR}", None),
                ("BIZ=$FOO-$ABC", None),
            ],
        );
    }

    #[test]
    fn incorrect_multiple_substitution_key_test() {
        check_test(
            &mut SubstitutionKeyChecker::default(),
            [
                (
                    "ABC=${BAR$XYZ}",
                    Some("The ABC key is not assigned properly"),
                ),
                (
                    "FOO=${ABC-$BAR}",
                    Some("The FOO key is not assigned properly"),
                ),
                (
                    "XYZ=${FOO${BAR}",
                    Some("The XYZ key is not assigned properly"),
                ),
            ],
        );
    }

    #[test]
    fn escaped_incorrect_substitution_key_test() {
        check_test(
            &mut SubstitutionKeyChecker::default(),
            [
                ("ABC=\\${BAR", None),
                ("FOO=\\$BAR}", None),
                ("FOO=\"\\${BAR\"", None),
                ("FOO=\"\\$BAR}", None),
            ],
        );
    }
}
