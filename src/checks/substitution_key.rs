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
            template: "The {} key isn't properly assigned",
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
            let raw_key = &value[index + 1..];

            if raw_key.contains('{') ^ raw_key.contains('}') {
                return Some(Warning::new(
                    line.clone(),
                    self.name,
                    self.message(line.get_key()?),
                ));
            }

            value = raw_key;
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
            (line_entry(3, 3, "FOO=$BAR"), None),
        ];

        run_substitution_tests(asserts);
    }

    #[test]
    fn incorrect_substitution_key_test() {
        let asserts = vec![
            (line_entry(1, 2, "ABC=${BAR"), Some(Warning::new(line_entry(1, 2, "ABC=${BAR"), "SubstitutionKey", "The ABC key isn't properly assigned"))),
            (line_entry(2, 2, "FOO=$BAR}"), Some(Warning::new(line_entry(2, 2, "FOO=$BAR}"), "SubstitutionKey", "The FOO key isn't properly assigned"))),
        ];

        run_substitution_tests(asserts);
    }
}
