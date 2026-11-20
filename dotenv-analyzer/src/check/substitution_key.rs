use dotenv_core::{LineEntry, closing_brace, is_escaped, substitution_key_name};

use super::Check;
use crate::{LintKind, Warning};

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

        // Checks if keys used in value have both '{' '}' or neither
        while let Some(index) = value.find('$') {
            let prefix = &value[..index];
            let raw_key = &value[index + 1..];

            let (is_incorrect_substitution, rest) = if raw_key.starts_with('{') {
                match closing_brace(raw_key) {
                    Some(end) => (
                        substitution_key_name(&raw_key[1..end]).is_none(),
                        &raw_key[end + 1..],
                    ),
                    None => (true, ""),
                }
            } else {
                // Separate initial key from the rest
                let (initial_key, rest) = raw_key
                    .find('$')
                    .map(|i| raw_key.split_at(i))
                    .unwrap_or((raw_key, ""));

                (initial_key.contains('}'), rest)
            };

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
    use crate::tests::check_test;

    #[test]
    fn correct_substitution_key_test() {
        check_test(
            &mut SubstitutionKeyChecker::default(),
            [
                ("ABC=$BAR", None),
                ("FOO=${BAR}", None),
                ("FOO=\"$BAR\"", None),
                ("FOO=${BAR}suffix", None),
                ("FOO='${BAR'", None),
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
                (
                    "FOO=${BAR:-baz}",
                    Some("The FOO key is not assigned properly"),
                ),
                (
                    "FOO=${MY.VAR}",
                    Some("The FOO key is not assigned properly"),
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
