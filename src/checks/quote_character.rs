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
    use crate::{check_tester, common::tests::*};

    const WARNING: &str = "The value has quote characters (\', \")";

    check_tester!{
        QuoteCharacterChecker;
        with_single_quote_test => {
            "FOO=BAR" => None,
            "FOO='BAR'" => Some(WARNING),
            "FOO='B\"AR'" => Some(WARNING),
            "FOO=\'BAR BAR\'" => None,
        },
        with_double_quote_test => {
            "FOO=BAR" => None,
            "FOO=\"BAR\"" => Some(WARNING),
            "FOO=\"BAR BAR\"" => None,
        },
        with_substitution_keys_test => {
            "BAR=\"$ABC\"" => None,
            "FOO='${BAR}BAR'" => None,
        },
        with_no_quotes_test => {
            "FOO=BAR" => None,
        }
    }
}
