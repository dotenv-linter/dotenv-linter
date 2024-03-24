use super::Check;
use crate::common::{LintKind, Warning};
use dotenv_lookup::LineEntry;

pub(crate) struct LeadingCharacterChecker<'a> {
    template: &'a str,
}

impl LeadingCharacterChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for LeadingCharacterChecker<'_> {
    fn default() -> Self {
        Self {
            template: "Invalid leading character detected",
        }
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
            Some(Warning::new(line.number, self.name(), self.message()))
        }
    }

    fn name(&self) -> LintKind {
        LintKind::LeadingCharacter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::check_test;

    const MESSAGE: &str = "Invalid leading character detected";

    #[test]
    fn no_leading_chars_test() {
        check_test(&mut LeadingCharacterChecker::default(), [("FOO=BAR", None)]);
    }

    #[test]
    fn blank_line() {
        check_test(&mut LeadingCharacterChecker::default(), [("", None)]);
    }

    #[test]
    fn leading_underscore() {
        check_test(
            &mut LeadingCharacterChecker::default(),
            [("_FOO=BAR", None)],
        );
    }

    #[test]
    fn leading_dot() {
        check_test(
            &mut LeadingCharacterChecker::default(),
            [(".FOO=BAR", Some(MESSAGE))],
        );
    }

    #[test]
    fn leading_asterisk() {
        check_test(
            &mut LeadingCharacterChecker::default(),
            [("*FOO=BAR", Some(MESSAGE))],
        );
    }

    #[test]
    fn leading_number() {
        check_test(
            &mut LeadingCharacterChecker::default(),
            [("1FOO=BAR", Some(MESSAGE))],
        );
    }

    #[test]
    fn leading_space() {
        check_test(
            &mut LeadingCharacterChecker::default(),
            [(" FOO=BAR", Some(MESSAGE))],
        );
    }

    #[test]
    fn two_leading_spaces() {
        check_test(
            &mut LeadingCharacterChecker::default(),
            [("  FOO=BAR", Some(MESSAGE))],
        );
    }

    #[test]
    fn leading_tab() {
        check_test(
            &mut LeadingCharacterChecker::default(),
            [("\tFOO=BAR", Some(MESSAGE))],
        );
    }
}
