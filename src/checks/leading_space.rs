use crate::checks::Lint;

pub struct LeadingSpaceCheck;

impl Lint for LeadingSpaceCheck {
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
    fn run() {
        let line = " DEBUG_HTTP=true";
        assert_eq!(Err(String::from("Leading space detected")), LeadingSpaceCheck.run(line));

        let line = "  DEBUG_HTTP=true";
        assert_eq!(Err(String::from("Leading space detected")), LeadingSpaceCheck.run(line));

        let line = "    DEBUG_HTTP=true";
        assert_eq!(Err(String::from("Leading space detected")), LeadingSpaceCheck.run(line));

        let line = "DEBUG_HTTP=true";
        assert_eq!(Ok(()), LeadingSpaceCheck.run(line));
    }
}
