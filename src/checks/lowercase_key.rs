use crate::checks::{Check, Warning};
use crate::LineEntry;

pub(crate) struct LowercaseKeyChecker {
    template: String,
}

impl Default for LowercaseKeyChecker {
    fn default() -> Self {
        Self {
            template: String::from("The {} key should be in uppercase"),
        }
    }
}

impl Check for LowercaseKeyChecker {
    fn run(&self, line: &LineEntry) -> Option<Warning> {
        let line_str: Vec<&str> = line.raw_string.split('=').collect();
        let key = line_str[0];
        if key.to_uppercase() == key.to_string() {
            None
        } else {
            Some(Warning {
                message: self.template.replace("{}", key),
            })
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
        assert_eq!(
            Some(Warning::new("The debug_http key should be in uppercase")),
            checker.run(line)
        );

        let line = &LineEntry {
            number: 1,
            raw_string: String::from("DEbUG_hTTP=true"),
        };
        assert_eq!(
            Some(Warning::new("The DEbUG_hTTP key should be in uppercase")),
            checker.run(line)
        );
    }
}
