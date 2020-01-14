use crate::common::*;

mod duplicated_keys;
mod incorrect_delimiter;
mod key_without_value;
mod leading_space;
mod lowercase_key;
mod spaces_around_equal;

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
    ]
}

pub fn run(file: FileEntry) -> Vec<Warning> {
    let mut checks = checklist();
    let mut warnings: Vec<Warning> = Vec::new();

    for line in file.lines {
        // TODO: Move to a method
        // A comment or empty line should just be skipped
        let trimmed_string = line.raw_string.trim();
        if trimmed_string.starts_with('#') || trimmed_string.is_empty() {
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
