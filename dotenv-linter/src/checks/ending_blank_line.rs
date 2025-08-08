use dotenv_lookup::LineEntry;

use super::Check;
use crate::common::{LintKind, Warning, LF};

pub(crate) struct EndingBlankLineChecker<'a> {
    template: &'a str,
}

impl EndingBlankLineChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for EndingBlankLineChecker<'_> {
    fn default() -> Self {
        Self {
            template: "No blank line at the end of the file",
        }
    }
}

impl Check for EndingBlankLineChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if line.is_last_line && !line.raw_string.ends_with(LF) {
            Some(Warning::new(line.number, self.name(), self.message()))
        } else {
            None
        }
    }

    fn name(&self) -> LintKind {
        LintKind::EndingBlankLine
    }

    fn skip_comments(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::check_test;

    #[test]
    fn blank_line() {
        check_test(&mut EndingBlankLineChecker::default(), [("\n", None)]);
    }

    #[test]
    fn blank_line_rn() {
        check_test(&mut EndingBlankLineChecker::default(), [("\r\n", None)]);
    }

    #[test]
    fn no_blank_line() {
        check_test(
            &mut EndingBlankLineChecker::default(),
            [("a", Some("No blank line at the end of the file"))],
        );
    }
}
