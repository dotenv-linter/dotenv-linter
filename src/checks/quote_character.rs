use crate::checks::Check;
use crate::common::*;

pub(crate) struct QuoteCharacterChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl QuoteCharacterChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for QuoteCharacterChecker<'_> {
    fn default() -> Self {
        Self {
            name: "QuoteCharacter",
            template: "The value has quote characters (\', \")",
        }
    }
}

impl Check for QuoteCharacterChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let val = line.get_value()?;
        if val.contains("\\n") || val.contains(char::is_whitespace) || val.contains('$') {
            return None;
        }

        if val.contains('\"') || val.contains('\'') {
            Some(Warning::new(line.clone(), self.name(), self.message()))
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    fn run_quote_char_tests(asserts: Vec<(LineEntry, Option<Warning>)>) {
        let mut checker = QuoteCharacterChecker::default();

        for assert in asserts {
            let (input, output) = assert;
            assert_eq!(checker.run(&input), output);
        }
    }

    #[test]
    fn with_single_quote_test() {
        let asserts = vec![
            (line_entry(1, 4, "FOO=BAR"), None),
            (
                line_entry(2, 4, "FOO='BAR'"),
                Some(Warning::new(
                    line_entry(2, 4, "FOO='BAR'"),
                    "QuoteCharacter",
                    "The value has quote characters (\', \")",
                )),
            ),
            (
                line_entry(3, 4, "FOO='B\"AR'"),
                Some(Warning::new(
                    line_entry(3, 4, "FOO='B\"AR'"),
                    "QuoteCharacter",
                    "The value has quote characters (\', \")",
                )),
            ),
            (line_entry(4, 4, "FOO=\'BAR BAR\'"), None),
        ];

        run_quote_char_tests(asserts);
    }

    #[test]
    fn with_double_quote_test() {
        let asserts = vec![
            (line_entry(1, 3, "FOO=BAR"), None),
            (
                line_entry(2, 3, "FOO=\"BAR\""),
                Some(Warning::new(
                    line_entry(2, 3, "FOO=\"BAR\""),
                    "QuoteCharacter",
                    "The value has quote characters (\', \")",
                )),
            ),
            (line_entry(3, 3, "FOO=\"BAR BAR\""), None),
        ];

        run_quote_char_tests(asserts);
    }

    #[test]
    fn with_substitution_keys_test() {
        let asserts = vec![
            (line_entry(1, 3, "FOO=BAR"), None),
            (line_entry(3, 3, "FOO=$BAR BAR"), None),
        ];

        run_quote_char_tests(asserts);
    }

    #[test]
    fn with_no_quotes_test() {
        let asserts = vec![(line_entry(1, 1, "FOO=BAR"), None)];

        run_quote_char_tests(asserts);
    }
}
