use crate::checks::Check;
use crate::common::*;

pub(crate) struct SpaceCharacterChecker<'a> {
    template: &'a str,
    name: &'a str,
}

impl SpaceCharacterChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for SpaceCharacterChecker<'_> {
    fn default() -> Self {
        Self {
            name: "SpaceCharacter",
            template: "The line has spaces around equal sign",
        }
    }
}

impl Check for SpaceCharacterChecker<'_> {
    fn run<'l>(&mut self, line: &'l LineEntry) -> Option<Warning<'l>> {
        let line_splitted = line.raw_string.split('=').collect::<Vec<&str>>();

        if let [key, value] = &line_splitted[..] {
            if key.ends_with(' ') || value.starts_with(' ') {
                return Some(Warning::new(line, self.name(), self.message()));
            }
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
    use crate::common::tests::*;

    const MESSAGE: &str = "The line has spaces around equal sign";

    #[test]
    fn working_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = line_entry(1, 1, "DEBUG_HTTP=true");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_leading_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = line_entry(1, 1, " DEBUG_HTTP=true");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_trailing_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = line_entry(1, 1, "DEBUG_HTTP=true ");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_empty_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = line_entry(1, 1, "");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_no_equal_sign_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = line_entry(1, 1, "DEBUG_HTTP true");
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn failing_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = line_entry(1, 1, "DEBUG-HTTP = true");
        let expected = Some(Warning::new(line.clone(), "SpaceCharacter", MESSAGE));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn failing_when_whitespace_before_equal_sign_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = line_entry(1, 1, "DEBUG-HTTP =true");
        let expected = Some(Warning::new(line.clone(), "SpaceCharacter", MESSAGE));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn failing_when_whitespace_after_equal_sign_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = line_entry(1, 1, "DEBUG-HTTP= true");
        let expected = Some(Warning::new(line.clone(), "SpaceCharacter", MESSAGE));
        assert_eq!(expected, checker.run(&line));
    }
}
