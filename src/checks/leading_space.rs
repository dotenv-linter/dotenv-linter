use crate::checks::Check;

pub(crate) struct LeadingSpaceChecker;

impl Check for LeadingSpaceChecker {
    fn run(&self, line: &str) -> Result<(), String> {
        if line.starts_with(' ') {
            Err(String::from("Leading space detected"))
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
        assert_eq!(Err(warning), LeadingSpaceChecker.run(line));

        let line = "  DEBUG_HTTP=true";
        assert_eq!(Err(warning), LeadingSpaceChecker.run(line));

        let line = "    DEBUG_HTTP=true";
        assert_eq!(Err(warning), LeadingSpaceChecker.run(line));
    }
}
