use crate::checks::Check;
use crate::common::*;
use crate::lints::LintKind;

pub(crate) struct EndingBlankLineChecker<'a> {
    template: &'a str,
}

impl Default for EndingBlankLineChecker<'_> {
    fn default() -> Self {
        Self {
            template: "No blank line at the end of the file",
        }
    }
}

impl EndingBlankLineChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Check for EndingBlankLineChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if line.is_last_line() && !line.raw_string.ends_with(LF) {
            Some(Warning::new(line.clone(), self.name(), self.message()))
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
    use crate::common::tests::*;

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
