use crate::checks::{Check, Warning};
use crate::LineEntry;

pub(crate) struct LeadingSpaceChecker {
    warning: Warning,
}

impl LeadingSpaceChecker {
    pub fn new() -> LeadingSpaceChecker {
        LeadingSpaceChecker {
            warning: Warning::new("Leading space detected"),
        }
    }
}

impl Check for LeadingSpaceChecker {
    fn run(&self, line: &LineEntry) -> Result<(), Warning> {
        if line.raw_string.starts_with(' ') {
            Err(self.warning.clone())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_space_check_run() {
        let line = "DEBUG_HTTP=true";
        assert_eq!(Ok(()), LeadingSpaceChecker.run(line));

        let warning = String::from("Leading space detected");
        let line = " DEBUG_HTTP=true";
        assert_eq!(Err(warning.to_owned()), LeadingSpaceChecker.run(line));

        let line = "  DEBUG_HTTP=true";
        assert_eq!(Err(warning.to_owned()), LeadingSpaceChecker.run(line));

        let line = "    DEBUG_HTTP=true";
        assert_eq!(Err(warning.to_owned()), LeadingSpaceChecker.run(line));
    }
}
