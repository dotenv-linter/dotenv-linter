use crate::checks::Check;
use crate::common::*;

pub(crate) struct LeadingCharacterChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl Default for LeadingCharacterChecker<'_> {
    fn default() -> Self {
        Self {
            name: "LeadingCharacter",
            template: "Invalid leading character detected",
        }
    }
}

impl LeadingCharacterChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Check for LeadingCharacterChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if line.is_empty()
            || line
                .raw_string
                .starts_with(|c: char| c.is_alphabetic() || c == '_')
        {
            None
        } else {
            Some(Warning::new(line.clone(), self.name(), self.message()))
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

    const MESSAGE: &str = "Invalid leading character detected";

    check_tester! {
        LeadingCharacterChecker;
        no_leading_chars_test => {
            "FOO=BAR" => None,
        },
        blank_line => {
            "" => None,
        },
        leading_underscore => {
            "_FOO=BAR" => None,
        },
        leading_dot => {
            ".FOO=BAR" => Some(MESSAGE),
        },
        leading_asterisk => {
            "*FOO=BAR" => Some(MESSAGE),
        },
        leading_number => {
            "1FOO=BAR" => Some(MESSAGE),
        },
        leading_space => {
            " FOO=BAR" => Some(MESSAGE),
        },
        two_leading_spaces => {
            "  FOO=BAR" => Some(MESSAGE),
        },
        leading_tab => {
            "\tFOO=BAR" => Some(MESSAGE),
        }
    }
}
