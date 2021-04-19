use crate::checks::Check;
use crate::common::*;

pub(crate) struct SubstitutionKeyChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl Default for SubstitutionKeyChecker<'_> {
    fn default() -> Self {
        Self {
            name: "SubstitutionKey",
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
                    line.clone(),
                    self.name,
                    self.message(line.get_key()?),
                ));
            }

            value = &rest;
        }
        None
    }

    fn name(&self) -> &str {
        self.name
    }
}

impl SubstitutionKeyChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", &key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    fn run_substitution_tests(asserts: Vec<(LineEntry, Option<Warning>)>) {
        let mut checker = SubstitutionKeyChecker::default();

        for assert in asserts {
            let (input, output) = assert;
            assert_eq!(checker.run(&input), output);
        }
    }

    #[test]
    fn correct_substitution_key_test() {
        let asserts = vec![
            (line_entry(1, 3, "ABC=$BAR"), None),
            (line_entry(2, 3, "FOO=${BAR}"), None),
            (line_entry(3, 3, "FOO=\"$BAR\""), None),
        ];

        run_substitution_tests(asserts);
    }

    #[test]
    fn incorrect_substitution_key_test() {
        let asserts = vec![
            (
                line_entry(1, 3, "ABC=${BAR"),
                Some(Warning::new(
                    line_entry(1, 3, "ABC=${BAR"),
                    "SubstitutionKey",
                    "The ABC key is not assigned properly",
                )),
            ),
            (
                line_entry(2, 3, "FOO=${BAR!}"),
                Some(Warning::new(
                    line_entry(2, 3, "FOO=${BAR!}"),
                    "SubstitutionKey",
                    "The FOO key is not assigned properly",
                )),
            ),
            (
                line_entry(3, 3, "XYZ=$BAR}"),
                Some(Warning::new(
                    line_entry(3, 3, "XYZ=$BAR}"),
                    "SubstitutionKey",
                    "The XYZ key is not assigned properly",
                )),
            ),
        ];

        run_substitution_tests(asserts);
    }

    #[test]
    fn multiple_substitution_key_test() {
        let asserts = vec![
            (line_entry(1, 3, "ABC=${BAR}$XYZ"), None),
            (line_entry(2, 3, "FOO=$ABC{${BAR}"), None),
            (line_entry(3, 3, "BIZ=$FOO-$ABC"), None),
        ];

        run_substitution_tests(asserts);
    }

    #[test]
    fn incorrect_multiple_substitution_key_test() {
        let asserts = vec![
            (
                line_entry(1, 3, "ABC=${BAR$XYZ}"),
                Some(Warning::new(
                    line_entry(1, 3, "ABC=${BAR$XYZ}"),
                    "SubstitutionKey",
                    "The ABC key is not assigned properly",
                )),
            ),
            (
                line_entry(2, 3, "FOO=${ABC-$BAR}"),
                Some(Warning::new(
                    line_entry(2, 3, "FOO=${ABC-$BAR}"),
                    "SubstitutionKey",
                    "The FOO key is not assigned properly",
                )),
            ),
            (
                line_entry(3, 3, "XYZ=${FOO${BAR}"),
                Some(Warning::new(
                    line_entry(3, 3, "XYZ=${FOO${BAR}"),
                    "SubstitutionKey",
                    "The XYZ key is not assigned properly",
                )),
            ),
        ];

        run_substitution_tests(asserts);
    }

    #[test]
    fn escaped_incorrect_substitution_key_test() {
        let asserts = vec![
            (line_entry(1, 4, "ABC=\\${BAR"), None),
            (line_entry(2, 4, "FOO=\\$BAR}"), None),
            (line_entry(3, 4, "FOO=\"\\${BAR\""), None),
            (line_entry(4, 4, "FOO=\"\\$BAR}"), None),
        ];

        run_substitution_tests(asserts);
    }
}
