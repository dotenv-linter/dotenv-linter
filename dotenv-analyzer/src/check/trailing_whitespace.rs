use dotenv_core::LineEntry;

use super::Check;
use crate::{LintKind, Warning};

pub(crate) struct TrailingWhitespaceChecker<'a> {
    template: &'a str,
}

impl TrailingWhitespaceChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for TrailingWhitespaceChecker<'_> {
    fn default() -> Self {
        Self {
            template: "Trailing whitespace detected",
        }
    }
}

impl Check for TrailingWhitespaceChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let raw_string = &line.raw_string;

        if raw_string.ends_with(' ') {
            return Some(Warning::new(line.number, self.name(), self.message()));
        }

        None
    }

    fn name(&self) -> LintKind {
        LintKind::TrailingWhitespace
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::check_test;

    const MESSAGE: &str = "Trailing whitespace detected";

    #[test]
    fn working_run() {
        check_test(
            &mut TrailingWhitespaceChecker::default(),
            [("DEBUG_HTTP=true", None)],
        );
    }

    #[test]
    fn failing_trailing_run() {
        check_test(
            &mut TrailingWhitespaceChecker::default(),
            [("DEBUG_HTTP=true  ", Some(MESSAGE))],
        );
    }
}
