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
    fn run<'l>(&mut self, line: &'l LineEntry) -> Option<Warning<'l>> {
        let raw_string = &line.raw_string;

        if raw_string.ends_with(' ') {
            return Some(Warning::new(line, self.name, self.message()));
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
        let mut checker = TrailingWhitespaceChecker::default();
        let line = line_entry(1, 1, "DEBUG_HTTP=true");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn failing_trailing_run() {
        let mut checker = TrailingWhitespaceChecker::default();
        let line = line_entry(1, 1, "DEBUG_HTTP=true  ");
        let expected = Some(Warning::new(line.clone(), "TrailingWhitespace", MESSAGE));
        assert_eq!(expected, checker.run(&line));
    }
}
