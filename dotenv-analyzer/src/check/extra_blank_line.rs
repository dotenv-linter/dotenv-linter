use dotenv_core::LineEntry;

use super::Check;
use crate::{LintKind, Warning};

pub(crate) struct ExtraBlankLineChecker<'a> {
    template: &'a str,
    last_blank_number: Option<usize>,
}

impl ExtraBlankLineChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for ExtraBlankLineChecker<'_> {
    fn default() -> Self {
        Self {
            template: "Extra blank line detected",
            last_blank_number: None,
        }
    }
}

impl Check for ExtraBlankLineChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if !line.is_empty() {
            return None;
        }

        let is_extra = self.last_blank_number.is_some_and(|n| n + 1 == line.number);

        self.last_blank_number = Some(line.number);

        if is_extra {
            return Some(Warning::new(line.number, self.name(), self.message()));
        }

        None
    }

    fn name(&self) -> LintKind {
        LintKind::ExtraBlankLine
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::check_test;

    #[test]
    fn no_blank_lines() {
        check_test(
            &mut ExtraBlankLineChecker::default(),
            [("A=B", None), ("C=D", None)],
        );
    }

    #[test]
    fn single_blank_line() {
        check_test(
            &mut ExtraBlankLineChecker::default(),
            [("A=B", None), ("", None), ("C=D", None)],
        );
    }

    #[test]
    fn two_blank_lines() {
        check_test(
            &mut ExtraBlankLineChecker::default(),
            [
                ("A=B", None),
                ("", None),
                ("", Some("Extra blank line detected")),
                ("C=D", None),
            ],
        );
    }

    #[test]
    fn three_blank_lines() {
        check_test(
            &mut ExtraBlankLineChecker::default(),
            [
                ("A=B", None),
                ("", None),
                ("", Some("Extra blank line detected")),
                ("", Some("Extra blank line detected")),
                ("C=D", None),
            ],
        );
    }
}
