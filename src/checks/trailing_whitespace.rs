use crate::checks::Check;
use crate::common::*;

pub(crate) struct TrailingWhitespaceChecker<'a> {
    template: &'a str,
    name: &'a str,
}

impl TrailingWhitespaceChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for TrailingWhitespaceChecker<'_> {
    fn default() -> Self {
        Self {
            name: "TrailingWhitespace",
            template: "Trailing whitespace detected",
        }
    }
}

impl Check for TrailingWhitespaceChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let raw_string = &line.raw_string;

        if raw_string.ends_with(' ') {
            return Some(Warning::new(line.clone(), self.name, self.message()));
        }

        None
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

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
