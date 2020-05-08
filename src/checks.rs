use crate::common::*;

mod duplicated_key;
mod ending_blank_line;
mod incorrect_delimiter;
mod key_without_value;
mod leading_character;
mod lowercase_key;
mod quote_character;
mod space_character;
mod unordered_key;

// This trait is used for checks which needs to know of only a single line
trait Check {
    fn run(&mut self, line: &LineEntry) -> Option<Warning>;
}

// Checklist for checks which needs to know of only a single line
fn checklist() -> Vec<Box<dyn Check>> {
    vec![
        Box::new(duplicated_key::DuplicatedKeyChecker::default()),
        Box::new(incorrect_delimiter::IncorrectDelimiterChecker::default()),
        Box::new(leading_character::LeadingCharacterChecker::default()),
        Box::new(key_without_value::KeyWithoutValueChecker::default()),
        Box::new(lowercase_key::LowercaseKeyChecker::default()),
        Box::new(quote_character::QuoteCharacterChecker::default()),
        Box::new(space_character::SpaceCharacterChecker::default()),
        Box::new(unordered_key::UnorderedKeyChecker::default()),
    ]
}

pub fn run(lines: Vec<LineEntry>) -> Vec<Warning> {
    let mut checks = checklist();
    let mut warnings: Vec<Warning> = Vec::new();

    if let Some((last_line, rest_lines)) = lines.split_last() {
        for line in rest_lines {
            if line.is_empty_or_comment() {
                continue;
            }

            for ch in &mut checks {
                if let Some(warning) = ch.run(line) {
                    warnings.push(warning);
                }
            }
        }

        let mut ending_line_checker = ending_blank_line::EndingBlankLineChecker::default();
        if let Some(warning) = ending_line_checker.run(last_line) {
            warnings.push(warning);
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn blank_entry(number: usize) -> LineEntry {
        LineEntry {
            number,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("\n"),
        }
    }

    fn line_entry(number: usize, str: &str) -> LineEntry {
        LineEntry {
            number,
            file_path: PathBuf::from(".env"),
            raw_string: String::from(str),
        }
    }

    #[test]
    fn run_with_empty_vec_test() {
        let empty: Vec<LineEntry> = Vec::new();
        let expected: Vec<Warning> = Vec::new();

        assert_eq!(expected, run(empty));
    }

    #[test]
    fn run_with_empty_line_test() {
        let lines: Vec<LineEntry> = vec![line_entry(1, ""), blank_entry(2)];
        let expected: Vec<Warning> = Vec::new();

        assert_eq!(expected, run(lines));
    }

    #[test]
    fn run_with_comment_line_test() {
        let lines: Vec<LineEntry> = vec![line_entry(1, "# Comment"), blank_entry(2)];
        let expected: Vec<Warning> = Vec::new();

        assert_eq!(expected, run(lines));
    }

    #[test]
    fn run_with_valid_line_test() {
        let lines: Vec<LineEntry> = vec![line_entry(1, "FOO=BAR"), blank_entry(2)];
        let expected: Vec<Warning> = Vec::new();

        assert_eq!(expected, run(lines));
    }

    #[test]
    fn run_with_invalid_line_test() {
        let line = line_entry(1, "FOO");
        let warning = Warning::new(
            line.clone(),
            String::from(
                "KeyWithoutValue: The FOO key should be with a value or have an equal sign",
            ),
        );
        let lines: Vec<LineEntry> = vec![line, blank_entry(2)];
        let expected: Vec<Warning> = vec![warning];

        assert_eq!(expected, run(lines));
    }

    #[test]
    fn run_without_blank_line_test() {
        let line = line_entry(1, "FOO=BAR");
        let warning = Warning::new(
            line.clone(),
            String::from("EndingBlankLine: No blank line at the end of the file"),
        );
        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = vec![warning];

        assert_eq!(expected, run(lines));
    }
}
