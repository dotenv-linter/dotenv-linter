use dotenv_lookup::LineEntry;

use super::Check;
use crate::common::{is_escaped, LintKind, Warning};

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

            // Separate initial key from the rest
            let (initial_key, rest) = raw_key
                .find('$')
                .map(|i| raw_key.split_at(i))
                .unwrap_or_else(|| (raw_key, ""));

            let end_brace_index = initial_key.find('}');
            let has_start_brace = initial_key.starts_with('{');
            let has_end_brace = end_brace_index.is_some();

            let incorrect_due_to_unbalanced = has_start_brace ^ has_end_brace;

            let incorrect_due_to_content = if has_start_brace && has_end_brace {
                let inner = &initial_key[1..end_brace_index.unwrap()];
                if let Some(colon_idx) = inner.find(':') {
                    let name = &inner[..colon_idx];
                    let rest = &inner[colon_idx + 1..];
                    let op = rest.chars().next();
                    let name_valid = !name.is_empty()
                        && name
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '_');
                    let op_valid = matches!(op, Some('-' | '+' | '=' | '?'));
                    !(name_valid && op_valid)
                } else {
                    // No operator, must be a pure NAME
                    let name = inner;
                    !( !name.is_empty()
                        && name
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '_') )
                }
            } else {
                false
            };

            let incorrect_missing_braces_with_operator = if !has_start_brace {
                // Scan variable name prefix
                let mut name_len = 0;
                for ch in initial_key.chars() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        name_len += ch.len_utf8();
                    } else {
                        break;
                    }
                }
                if name_len > 0 {
                    let remainder = &initial_key[name_len..];
                    // Any use of ':' operator after $NAME without braces is invalid
                    remainder.starts_with(':')
                } else {
                    false
                }
            } else {
                false
            };

            let is_incorrect_substitution = incorrect_due_to_unbalanced
                || incorrect_due_to_content
                || (incorrect_missing_braces_with_operator && !is_escaped(prefix));

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
                ("FOO=${VAR:-def}", None),
                ("FOO=${VAR:=def}", None),
                ("FOO=${VAR:+alt}", None),
                ("FOO=${VAR:?err}", None),
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
                    "FOO=$BAR:default",
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
