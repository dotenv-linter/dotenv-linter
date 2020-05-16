use crate::checks::Check;
use crate::common::*;

pub(crate) struct TrailingWhitespaceChecker {
    template: String,
    name: String,
}

impl TrailingWhitespaceChecker {
    fn message(&self) -> String {
        return format!("{}: {}", self.name, self.template);
    }
}

impl Default for TrailingWhitespaceChecker {
    fn default() -> Self {
        Self {
            name: String::from("TrailingWhitespace"),
            template: String::from("The line has trailing whitespace")
        }
    }
}

impl Check for TrailingWhitespaceChecker {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let raw_string = &line.raw_string;

        if raw_string.ends_with(' ') {
            let warning = Warning::new(line.clone(), self.message());
            return Some(warning);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const MESSAGE: &str = "TrailingWhitespace: The line has trailing whitespace";

    #[test]
    fn working_run() {
        let mut checker = TrailingWhitespaceChecker::default();

        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn failing_trailing_run() {
        let mut checker = TrailingWhitespaceChecker::default();
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("DEBUG_HTTP=true  "),
        };

        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(&line));
    }
}
