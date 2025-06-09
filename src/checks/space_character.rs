use super::Check;
use crate::common::{LintKind, Warning};
use dotenv_lookup::LineEntry;

pub(crate) struct SpaceCharacterChecker<'a> {
    template: &'a str,
}

impl SpaceCharacterChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for SpaceCharacterChecker<'_> {
    fn default() -> Self {
        Self {
            template: "The line has spaces around equal sign",
        }
    }
}

impl Check for SpaceCharacterChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let line_splitted = line.raw_string.split('=').collect::<Vec<&str>>();

        if let [key, value] = &line_splitted[..] {
            if key.ends_with(' ') || value.starts_with(' ') {
                return Some(Warning::new(line.number, self.name(), self.message()));
            }
        }

        None
    }

    fn name(&self) -> LintKind {
        LintKind::SpaceCharacter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::check_test;

    const MESSAGE: &str = "The line has spaces around equal sign";

    #[test]
    fn working_run() {
        check_test(
            &mut SpaceCharacterChecker::default(),
            [("DEBUG_HTTP=true", None)],
        );
    }

    #[test]
    fn working_leading_run() {
        check_test(
            &mut SpaceCharacterChecker::default(),
            [(" DEBUG_HTTP=true", None)],
        );
    }

    #[test]
    fn working_trailing_run() {
        check_test(
            &mut SpaceCharacterChecker::default(),
            [("DEBUG_HTTP=true ", None)],
        );
    }

    #[test]
    fn working_empty_run() {
        check_test(&mut SpaceCharacterChecker::default(), [("", None)]);
    }

    #[test]
    fn working_no_equal_sign_run() {
        check_test(
            &mut SpaceCharacterChecker::default(),
            [("DEBUG_HTTP true", None)],
        );
    }

    #[test]
    fn failing_run() {
        check_test(
            &mut SpaceCharacterChecker::default(),
            [("DEBUG_HTTP = true", Some(MESSAGE))],
        );
    }

    #[test]
    fn failing_when_whitespace_before_equal_sign_run() {
        check_test(
            &mut SpaceCharacterChecker::default(),
            [("DEBUG_HTTP =true", Some(MESSAGE))],
        );
    }

    #[test]
    fn failing_when_whitespace_after_equal_sign_run() {
        check_test(
            &mut SpaceCharacterChecker::default(),
            [("DEBUG_HTTP= true", Some(MESSAGE))],
        );
    }
}
