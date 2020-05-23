use crate::common::*;

mod duplicated_key;
mod ending_blank_line;
mod extra_blank_line;
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
    fn name(&self) -> &str;
}

// Checklist for checks which needs to know of only a single line
fn checklist() -> Vec<Box<dyn Check>> {
    vec![
        Box::new(duplicated_key::DuplicatedKeyChecker::default()),
        Box::new(extra_blank_line::ExtraBlankLineChecker::default()),
        Box::new(incorrect_delimiter::IncorrectDelimiterChecker::default()),
        Box::new(leading_character::LeadingCharacterChecker::default()),
        Box::new(key_without_value::KeyWithoutValueChecker::default()),
        Box::new(lowercase_key::LowercaseKeyChecker::default()),
        Box::new(quote_character::QuoteCharacterChecker::default()),
        Box::new(space_character::SpaceCharacterChecker::default()),
        Box::new(unordered_key::UnorderedKeyChecker::default()),
    ]
}

pub fn check_names() -> Vec<String> {
    checklist().iter()
        .map(|check| check.name().to_string())
        .collect()
}

pub fn run(lines: Vec<LineEntry>, skip_checks: &[&str]) -> Vec<Warning> {
    let mut checks = checklist();
    checks.retain(|c| !skip_checks.contains(&c.name()));

    let mut warnings: Vec<Warning> = Vec::new();

    for line in &lines {
        if line.is_comment() {
            continue;
        }

        for ch in &mut checks {
            if let Some(warning) = ch.run(line) {
                warnings.push(warning);
            }
        }
    }

    if let Some(last_line) = lines.last() {
        let mut ending_line_checker = ending_blank_line::EndingBlankLineChecker::default();

        if !skip_checks.contains(&ending_line_checker.name()) {
            if let Some(warning) = ending_line_checker.run(last_line) {
                warnings.push(warning);
            }
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn blank_line_entry(number: usize) -> LineEntry {
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
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(empty, &skip_checks));
    }

    #[test]
    fn run_with_empty_line_test() {
        let lines: Vec<LineEntry> = vec![blank_line_entry(1)];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(lines, &skip_checks));
    }

    #[test]
    fn run_with_comment_line_test() {
        let lines: Vec<LineEntry> = vec![line_entry(1, "# Comment = 'Value'"), blank_line_entry(2)];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(lines, &skip_checks));
    }

    #[test]
    fn run_with_valid_line_test() {
        let lines: Vec<LineEntry> = vec![line_entry(1, "FOO=BAR"), blank_line_entry(2)];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(lines, &skip_checks));
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
        let lines: Vec<LineEntry> = vec![line, blank_line_entry(2)];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(lines, &skip_checks));
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
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(lines, &skip_checks));
    }

    #[test]
    fn skip_one_check() {
        let line1 = line_entry(1, "FOO");
        let line2 = line_entry(2, "1FOO");
        let warning = Warning::new(
            line2.clone(),
            String::from("LeadingCharacter: Invalid leading character detected"),
        );
        let lines: Vec<LineEntry> = vec![line1, line2, blank_line_entry(3)];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = vec!["KeyWithoutValue"];

        assert_eq!(expected, run(lines, &skip_checks));
    }

    #[test]
    fn skip_all_checks() {
        let line = line_entry(1, "FOO");
        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = vec!["KeyWithoutValue", "EndingBlankLine"];

        assert_eq!(expected, run(lines, &skip_checks));
    }
}
