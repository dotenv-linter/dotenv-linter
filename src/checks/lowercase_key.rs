use crate::checks::{Check, Warning};
use crate::LineEntry;

pub(crate) struct LowercaseKeyChecker {
    warning: Warning,
}

impl Default for LowercaseKeyChecker {
    fn default() -> Self {
        Self {
            warning: Warning::new("Key contains lowercase chars"),
        }
    }
}

impl Check for LowercaseKeyChecker {
    fn run(&self, line: &LineEntry) -> Option<Warning> {
        let line_str: Vec<&str> = line.raw_string.split("=").collect();
        if line_str[0].to_uppercase() == line_str[0] {
            None
        } else {
            Some(self.warning.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_key_checker_run() {
        let checker = LowercaseKeyChecker::default();
        let line = &LineEntry {
            number: 1,
            raw_string: String::from("DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(line));

        let line = &LineEntry {
            number: 1,
            raw_string: String::from("debug_http=true"),
        };
        assert_eq!(Some(checker.warning.to_owned()), checker.run(line));

        let line = &LineEntry {
            number: 1,
            raw_string: String::from("DEbUG_hTTP=true"),
        };
        assert_eq!(Some(checker.warning.to_owned()), checker.run(line));
    }
}
