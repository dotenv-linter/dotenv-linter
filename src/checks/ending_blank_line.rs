use crate::checks::Check;
use crate::common::*;

pub(crate) struct EndingBlankLineChecker {
    name: &'static str,
    template: &'static str,
}

impl Default for EndingBlankLineChecker {
    fn default() -> Self {
        Self {
            name: "EndingBlankLine",
            template: "No blank line at the end of the file",
        }
    }
}

impl EndingBlankLineChecker {
    fn message(&self) -> String {
        format!("{}: {}", self.name, self.template)
    }
}

impl Check for EndingBlankLineChecker {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if line.raw_string.ends_with('\n') {
            None
        } else {
            Some(Warning::new(line.clone(), self.message()))
        }
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn blank_line() {
        let mut checker = EndingBlankLineChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("\n"),
        };

        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn blank_line_rn() {
        let mut checker = EndingBlankLineChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("\r\n"),
        };

        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn no_blank_line() {
        let mut checker = EndingBlankLineChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("a"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            String::from("EndingBlankLine: No blank line at the end of the file"),
        ));

        assert_eq!(expected, checker.run(&line));
    }
}
