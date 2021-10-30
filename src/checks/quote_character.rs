use super::Check;
use crate::common::{LineEntry, LintKind, Warning};

pub(crate) struct QuoteCharacterChecker<'a> {
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
            Some(Warning::new(line.number, self.name(), self.message()))
        } else {
            None
        }
    }

    fn name(&self) -> LintKind {
        LintKind::QuoteCharacter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::check_test;

    const WARNING: &str = "The value has quote characters (\', \")";

    #[test]
    fn with_single_quote_test() {
        check_test(
            &mut QuoteCharacterChecker::default(),
            [
                ("FOO=BAR", None),
                ("FOO='BAR'", Some(WARNING)),
                ("FOO='B\"AR'", Some(WARNING)),
                ("FOO=\'BAR BAR\'", None),
            ],
        );
    }

    #[test]
    fn with_double_quote_test() {
        check_test(
            &mut QuoteCharacterChecker::default(),
            [
                ("FOO=BAR", None),
                ("FOO=\"Bar\"", Some(WARNING)),
                ("FOO=\"BAR BAR\"", None),
            ],
        );
    }

    #[test]
    fn with_substitution_keys_test() {
        check_test(
            &mut QuoteCharacterChecker::default(),
            [("BAR=\"$ABC\"", None), ("FOO='${BAR}BAR'", None)],
        );
    }

    #[test]
    fn with_no_quotes_test() {
        check_test(&mut QuoteCharacterChecker::default(), [("FOO=BAR", None)]);
    }
}
