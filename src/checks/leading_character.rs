use crate::checks::Check;
use crate::common::*;

pub(crate) struct LeadingCharacterChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl Default for LeadingCharacterChecker<'_> {
    fn default() -> Self {
        Self {
            name: "LeadingCharacter",
            template: "Invalid leading character detected",
        }
    }
}

impl LeadingCharacterChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Check for LeadingCharacterChecker<'_> {
    fn run<'l>(&mut self, line: &'l LineEntry) -> Option<Warning<'l>> {
        if line.is_empty()
            || line
                .raw_string
                .starts_with(|c: char| c.is_alphabetic() || c == '_')
        {
            None
        } else {
            Some(Warning::new(line, self.name(), self.message()))
        }
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    const MESSAGE: &str = "Invalid leading character detected";

    #[test]
    fn no_leading_chars_test() {
        let mut checker = LeadingCharacterChecker::default();
        let line = line_entry(1, 1, "FOO=BAR");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn blank_line() {
        let mut checker = LeadingCharacterChecker::default();
        let line = line_entry(1, 1, "");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn leading_underscore() {
        let mut checker = LeadingCharacterChecker::default();
        let line = line_entry(1, 1, "_FOO=BAR");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn leading_dot() {
        let mut checker = LeadingCharacterChecker::default();
        let line = line_entry(1, 1, ".FOO=BAR");
        assert_eq!(
            Some(Warning::new(line.clone(), "LeadingCharacter", MESSAGE)),
            checker.run(&line)
        );
    }

    #[test]
    fn leading_asterisk() {
        let mut checker = LeadingCharacterChecker::default();
        let line = line_entry(1, 1, "*FOO=BAR");
        assert_eq!(
            Some(Warning::new(line.clone(), "LeadingCharacter", MESSAGE)),
            checker.run(&line)
        );
    }

    #[test]
    fn leading_number() {
        let mut checker = LeadingCharacterChecker::default();
        let line = line_entry(1, 1, "1FOO=BAR");
        assert_eq!(
            Some(Warning::new(line.clone(), "LeadingCharacter", MESSAGE)),
            checker.run(&line)
        );
    }

    #[test]
    fn leading_space() {
        let mut checker = LeadingCharacterChecker::default();
        let line = line_entry(1, 1, " FOO=BAR");
        let expected = Some(Warning::new(line.clone(), "LeadingCharacter", MESSAGE));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn two_leading_spaces() {
        let mut checker = LeadingCharacterChecker::default();
        let line = line_entry(1, 1, "  FOO=BAR");
        let expected = Some(Warning::new(line.clone(), "LeadingCharacter", MESSAGE));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn leading_tab() {
        let mut checker = LeadingCharacterChecker::default();
        let line = line_entry(1, 1, "\tFOO=BAR");
        let expected = Some(Warning::new(line.clone(), "LeadingCharacter", MESSAGE));
        assert_eq!(expected, checker.run(&line));
    }
}
