use crate::checks::Check;
use crate::common::*;

pub(crate) struct ExtraBlankLineChecker<'a> {
    template: &'a str,
    name: &'a str,
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
            name: "ExtraBlankLine",
            last_blank_number: None,
        }
    }
}

impl Check for ExtraBlankLineChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if !line.is_empty() {
            return None;
        }

        let is_extra = self
            .last_blank_number
            .map_or(false, |n| n + 1 == line.number);

        self.last_blank_number = Some(line.number);

        if is_extra {
            return Some(Warning::new(line.clone(), self.name(), self.message()));
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

    fn run_asserts(asserts: Vec<(&str, Option<&str>)>) {
        let mut checker = ExtraBlankLineChecker::default();

        for (i, assert) in asserts.iter().enumerate() {
            let (content, message) = *assert;
            let line = line_entry(i + 1, asserts.len(), content);

            let expected = message.map(|msg| Warning::new(line.clone(), "ExtraBlankLine", msg));

            assert_eq!(checker.run(&line), expected);
        }
    }

    #[test]
    fn no_blank_lines() {
        let asserts = vec![("A=B", None), ("C=D", None)];

        run_asserts(asserts);
    }

    #[test]
    fn single_blank_line() {
        let asserts = vec![("A=B", None), ("", None), ("C=D", None)];

        run_asserts(asserts);
    }

    #[test]
    fn two_blank_lines() {
        let asserts = vec![
            ("A=B", None),
            ("", None),
            ("", Some("Extra blank line detected")),
            ("C=D", None),
        ];

        run_asserts(asserts);
    }

    #[test]
    fn three_blank_lines() {
        let asserts = vec![
            ("A=B", None),
            ("", None),
            ("", Some("Extra blank line detected")),
            ("", Some("Extra blank line detected")),
            ("C=D", None),
        ];

        run_asserts(asserts);
    }
}
