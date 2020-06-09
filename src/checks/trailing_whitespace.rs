use crate::checks::Check;
use crate::common::*;

pub(crate) struct TrailingWhitespaceChecker<'a> {
    template: &'a str,
    name: &'a str,
}

impl TrailingWhitespaceChecker<'_> {
    fn message(&self) -> String {
        return format!("{}: {}", self.name, self.template);
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
            return Some(Warning::new(line.clone(), self.message()));
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
    use std::path::PathBuf;

    const MESSAGE: &str = "TrailingWhitespace: Trailing whitespace detected";

    #[test]
    fn working_run() {
        let mut checker = TrailingWhitespaceChecker::default();

        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn failing_trailing_run() {
        let mut checker = TrailingWhitespaceChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("DEBUG_HTTP=true  "),
        };

        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(&line));
    }
}
