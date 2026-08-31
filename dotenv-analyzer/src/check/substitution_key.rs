use dotenv_core::{LineEntry, is_escaped};

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

            if is_escaped(prefix) {
                value = raw_key;
                continue;
            }

            let (is_incorrect_substitution, rest) = if let Some(braced_key) =
                raw_key.strip_prefix('{')
            {
                match braced_key.find('}') {
                    Some(end_brace_index) => {
                        let key = substitution_key_name(&braced_key[..end_brace_index]);
                        let is_invalid_key = key.is_none_or(|key| {
                            key.is_empty()
                                || key.contains(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                        });
                        (is_invalid_key, &braced_key[(end_brace_index + 1)..])
                    }
                    None => (true, ""),
                }
            } else {
                let (initial_key, rest) = raw_key
                    .find('$')
                    .map(|i| raw_key.split_at(i))
                    .unwrap_or_else(|| (raw_key, ""));

                (initial_key.contains('}'), rest)
            };

            if is_incorrect_substitution {
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

fn substitution_key_name(key: &str) -> Option<&str> {
    let mut operator_index = None;

    for operator in [":-", ":+", ":=", ":?"] {
        for (index, _) in key.match_indices(operator) {
            if operator_index.is_some() {
                return None;
            }
            operator_index = Some((index, operator));
        }
    }

    match operator_index {
        Some((index, ":-" | ":+")) => Some(&key[..index]),
        Some(_) => None,
        None => Some(key),
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
                ("FOO=${NODE_ENV:-development if not set}", None),
                ("BAR=${CI:+only when running in CI}", None),
                ("BAZ=${APP_ENV:-$NODE_ENV}", None),
                ("FOO=${NODE_ENV:-development}${CI:+only}", None),
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
                (
                    "QUX=${NODE_ENV:development}",
                    Some("The QUX key is not assigned properly"),
                ),
                (
                    "BAZ=${SOME_VALUE--$$++???_END}",
                    Some("The BAZ key is not assigned properly"),
                ),
                (
                    "BUS=${CI:+only:?IS_UNSET:other}",
                    Some("The BUS key is not assigned properly"),
                ),
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
