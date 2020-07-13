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
    fn message(&self) -> String {
        String::from(self.template)
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

    use std::path::PathBuf;

    #[test]
    fn blank_line() {
        let mut checker = EndingBlankLineChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("\n"),
        };

        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn blank_line_rn() {
        let mut checker = EndingBlankLineChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("\r\n"),
        };

        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn no_blank_line() {
        let mut checker = EndingBlankLineChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("a"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            "EndingBlankLine",
            String::from("No blank line at the end of the file"),
        ));

        assert_eq!(expected, checker.run(&line));
    }
}
