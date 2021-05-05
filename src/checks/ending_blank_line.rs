use crate::checks::Check;
use crate::common::*;

pub(crate) struct EndingBlankLineChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl Default for EndingBlankLineChecker<'_> {
    fn default() -> Self {
        Self {
            name: "EndingBlankLine",
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
    fn run<'l>(&mut self, line: &'l LineEntry) -> Option<Warning<'l>> {
        if line.is_last_line() && !line.raw_string.ends_with(LF) {
            Some(Warning::new(line, self.name(), self.message()))
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        self.name
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
        let mut checker = EndingBlankLineChecker::default();
        let line = line_entry(1, 1, "\n");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn blank_line_rn() {
        let mut checker = EndingBlankLineChecker::default();
        let line = line_entry(1, 1, "\r\n");

        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn no_blank_line() {
        let mut checker = EndingBlankLineChecker::default();
        let line = line_entry(1, 1, "a");
        let expected = Some(Warning::new(
            line.clone(),
            "EndingBlankLine",
            "No blank line at the end of the file",
        ));

        assert_eq!(expected, checker.run(&line));
    }
}
