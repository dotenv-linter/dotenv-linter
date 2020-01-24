use crate::common::*;

mod duplicated_keys;
mod incorrect_delimiter;
mod key_without_value;
mod leading_space;
mod lowercase_key;
mod spaces_around_equal;
mod unordered_keys;

// This trait is used for checks which needs to know of only a single line
trait Check {
    fn run(&mut self, line: LineEntry) -> Option<Warning>;
}

// Checklist for checks which needs to know of only a single line
fn checklist() -> Vec<Box<dyn Check>> {
    vec![
        Box::new(duplicated_keys::DuplicatedKeysChecker::default()),
        Box::new(incorrect_delimiter::IncorrectDelimiterChecker::default()),
        Box::new(leading_space::LeadingSpaceChecker::default()),
        Box::new(key_without_value::KeyWithoutValueChecker::default()),
        Box::new(lowercase_key::LowercaseKeyChecker::default()),
        Box::new(spaces_around_equal::SpacesAroundEqualChecker::default()),
        Box::new(unordered_keys::UnorderedKeysChecker::default()),
    ]
}

pub fn run(lines: Vec<LineEntry>) -> Vec<Warning> {
    let mut checks = checklist();
    let mut warnings: Vec<Warning> = Vec::new();

    for line in lines {
        if line.should_be_skipped() {
            continue;
        }

        for ch in &mut checks {
            // TODO: Use a reference instead of the clone method
            if let Some(warning) = ch.run(line.clone()) {
                warnings.push(warning);
            }
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_with_empty_vec_test() {
        let empty: Vec<LineEntry> = Vec::new();
        let expected: Vec<Warning> = Vec::new();

        assert_eq!(expected, run(empty));
    }

    #[test]
    fn run_with_empty_line_test() {
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from(""),
        };

        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = Vec::new();

        assert_eq!(expected, run(lines));
    }

    #[test]
    fn run_with_comment_line_test() {
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("# Comment"),
        };

        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = Vec::new();

        assert_eq!(expected, run(lines));
    }

    #[test]
    fn run_with_valid_line_test() {
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOO=BAR"),
        };

        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = Vec::new();

        assert_eq!(expected, run(lines));
    }

    #[test]
    fn run_with_invalid_line_test() {
        let line = LineEntry {
            number: 1,
            file_name: String::from(".env"),
            raw_string: String::from("FOO"),
        };
        let warning = Warning::new(
            line.clone(),
            String::from("The FOO key should be with a value or have an equal sign"),
        );
        let lines: Vec<LineEntry> = vec![line];
        let expected: Vec<Warning> = vec![warning];

        assert_eq!(expected, run(lines));
    }
}
