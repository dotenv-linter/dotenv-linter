use crate::checks::Check;
use crate::common::*;

pub(crate) struct ExtraBlankLineChecker<'a> {
    template: &'a str,
    name: &'a str,
    last_blank_number: Option<usize>,
}

impl ExtraBlankLineChecker<'_> {
    fn message(&self) -> String {
        return format!("{}: {}", self.name, self.template);
    }
}

impl Default for ExtraBlankLineChecker<'_> {
    fn default() -> Self {
        Self {
            name: "ExtraBlankLine",
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

        if let Some(last_blank_number) = self.last_blank_number {
            if last_blank_number + 1 == line.number {
                return Some(Warning::new(line.clone(), self.message()));
            }
        }
        self.last_blank_number = Some(line.number);

        None
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn run_asserts(asserts: Vec<(&str, Option<&str>)>) {
        let mut checker = ExtraBlankLineChecker::default();

        for (i, assert) in asserts.iter().enumerate() {
            let (content, message) = *assert;
            let line = LineEntry {
                number: i + 1,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                },
                raw_string: String::from(content),
            };
            let expected = message.map(|msg| Warning::new(line.clone(), String::from(msg)));

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
            ("", Some("ExtraBlankLine: Extra blank line detected")),
            ("C=D", None),
        ];

        run_asserts(asserts);
    }
}
