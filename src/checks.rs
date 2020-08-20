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
mod trailing_whitespace;
mod unordered_key;

// This trait is used for checks which needs to know of only a single line
trait Check {
    fn run(&mut self, line: &LineEntry) -> Option<Warning>;
    fn name(&self) -> &str;
    fn skip_comments(&self) -> bool {
        true
    }
}

// Checklist for checks which needs to know of only a single line
fn checklist() -> Vec<Box<dyn Check>> {
    vec![
        Box::new(duplicated_key::DuplicatedKeyChecker::default()),
        Box::new(ending_blank_line::EndingBlankLineChecker::default()),
        Box::new(extra_blank_line::ExtraBlankLineChecker::default()),
        Box::new(incorrect_delimiter::IncorrectDelimiterChecker::default()),
        Box::new(leading_character::LeadingCharacterChecker::default()),
        Box::new(key_without_value::KeyWithoutValueChecker::default()),
        Box::new(lowercase_key::LowercaseKeyChecker::default()),
        Box::new(quote_character::QuoteCharacterChecker::default()),
        Box::new(space_character::SpaceCharacterChecker::default()),
        Box::new(trailing_whitespace::TrailingWhitespaceChecker::default()),
        Box::new(unordered_key::UnorderedKeyChecker::default()),
    ]
}

pub fn available_check_names() -> Vec<String> {
    checklist()
        .iter()
        .map(|check| check.name().to_string())
        .collect()
}

pub fn run(lines: &[LineEntry], skip_checks: &[&str]) -> Vec<Warning> {
    let mut checks = checklist();

    // Skip checks with the --skip argument (globally)
    checks.retain(|c| !skip_checks.contains(&c.name()));

    // Skip checks with comments (dotenv-linter:on/off)
    let mut disabled_checks: Vec<&str> = Vec::new();

    let mut warnings: Vec<Warning> = Vec::new();

    for line in lines {
        let is_comment = line.is_comment();
        if is_comment {
            if let Some(comment) = comment::parse(&line.raw_string) {
                if comment.is_disabled() {
                    // Disable checks from a comment using the dotenv-linter:off flag
                    disabled_checks.extend(comment.checks);
                } else {
                    // Enable checks if the comment has the dotenv-linter:on flag
                    disabled_checks.retain(|&s| !comment.checks.contains(&s));
                }
            }
        }

        for ch in &mut checks {
            if is_comment && ch.skip_comments() {
                continue;
            }

            if disabled_checks.contains(&ch.name()) {
                continue;
            }

            if let Some(warning) = ch.run(line) {
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

    fn blank_line_entry(number: usize, total_lines: usize) -> LineEntry {
        LineEntry {
            number,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines,
            },
            raw_string: String::from("\n"),
        }
    }

    fn line_entry(number: usize, total_lines: usize, str: &str) -> LineEntry {
        LineEntry {
            number,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines,
            },
            raw_string: String::from(str),
        }
    }

    #[test]
    fn run_with_empty_vec_test() {
        let empty: Vec<LineEntry> = Vec::new();
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(&empty, &skip_checks));
    }

    #[test]
    fn run_with_empty_line_test() {
        let lines: Vec<LineEntry> = vec![blank_line_entry(1, 1)];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn run_with_comment_line_test() {
        let lines: Vec<LineEntry> = vec![
            line_entry(1, 2, "# Comment = 'Value'"),
            blank_line_entry(2, 2),
        ];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn run_with_valid_line_test() {
        let lines: Vec<LineEntry> = vec![line_entry(1, 2, "FOO=BAR"), blank_line_entry(2, 2)];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn run_with_invalid_line_test() {
        let line = line_entry(1, 2, "FOO");
        let warning = Warning::new(
            line.clone(),
            "KeyWithoutValue",
            String::from("The FOO key should be with a value or have an equal sign"),
        );
        let lines: Vec<LineEntry> = vec![line, blank_line_entry(2, 2)];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn run_without_blank_line_test() {
        let line = line_entry(1, 1, "FOO=BAR");
        let warning = Warning::new(
            line.clone(),
            "EndingBlankLine",
            String::from("No blank line at the end of the file"),
        );
        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn skip_one_check() {
        let line1 = line_entry(1, 3, "FOO\n");
        let line2 = line_entry(2, 3, "1FOO\n");
        let warning = Warning::new(
            line2.clone(),
            "LeadingCharacter",
            String::from("Invalid leading character detected"),
        );
        let lines: Vec<LineEntry> = vec![line1, line2, blank_line_entry(3, 3)];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = vec!["KeyWithoutValue"];

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn skip_all_checks() {
        let line = line_entry(1, 1, "FOO");
        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = Vec::new();
        let skip_checks: Vec<&str> = vec!["KeyWithoutValue", "EndingBlankLine"];

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn skip_one_check_via_comment() {
        let line1 = line_entry(1, 4, "# dotenv-linter:off KeyWithoutValue\n");
        let line2 = line_entry(2, 4, "FOO\n");
        let line3 = line_entry(3, 4, "1FOO\n");
        let warning = Warning::new(
            line3.clone(),
            "LeadingCharacter",
            String::from("Invalid leading character detected"),
        );
        let lines: Vec<LineEntry> = vec![line1, line2, line3, blank_line_entry(4, 4)];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn skip_collision() {
        let line1 = line_entry(1, 4, "# dotenv-linter:on KeyWithoutValue\n");
        let line2 = line_entry(2, 4, "FOO\n");
        let line3 = line_entry(3, 4, "1FOO\n");
        let warning = Warning::new(
            line3.clone(),
            "LeadingCharacter",
            String::from("Invalid leading character detected"),
        );
        let lines: Vec<LineEntry> = vec![line1, line2, line3, blank_line_entry(4, 4)];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = vec!["KeyWithoutValue"];

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn on_and_off_same_checks() {
        let line1 = line_entry(
            1,
            5,
            "# dotenv-linter:off KeyWithoutValue, LeadingCharacter\n",
        );
        let line2 = line_entry(2, 5, "FOO\n");
        let line3 = line_entry(3, 5, "# dotenv-linter:on LeadingCharacter\n");
        let line4 = line_entry(4, 5, "1FOO\n");
        let warning = Warning::new(
            line4.clone(),
            "LeadingCharacter",
            String::from("Invalid leading character detected"),
        );
        let lines: Vec<LineEntry> = vec![line1, line2, line3, line4, blank_line_entry(5, 5)];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn only_simple_comment() {
        let line = line_entry(1, 1, "# Simple comment");
        let warning = Warning::new(
            line.clone(),
            "EndingBlankLine",
            String::from("No blank line at the end of the file"),
        );
        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = vec![warning];
        let skip_checks: Vec<&str> = Vec::new();

        assert_eq!(expected, run(&lines, &skip_checks));
    }

    #[test]
    fn check_name_list() {
        let available_check_names = available_check_names();
        for check in checklist() {
            let check_name = check.name();
            assert!(available_check_names.contains(&check_name.to_string()));
        }
    }
}
