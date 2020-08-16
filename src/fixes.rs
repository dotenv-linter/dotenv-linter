use crate::common::*;

mod ending_blank_line;
mod key_without_value;
mod lowercase_key;
mod space_character;
mod trailing_whitespace;

trait Fix {
    fn name(&self) -> &str;

    fn fix_warnings(
        &self,
        warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        let mut count: usize = 0;
        for warning in warnings {
            let line = lines.get_mut(warning.line_number() - 1)?;
            if self.fix_line(line).is_some() {
                warning.mark_as_fixed();
                count += 1;
            }
        }

        Some(count)
    }

    fn fix_line(&self, _line: &mut LineEntry) -> Option<()> {
        None
    }
}

// TODO: skip fixes (like checks)
// The fix order is matter
fn fixlist() -> Vec<Box<dyn Fix>> {
    vec![
        // At first we run the fixers that handle a single line entry (they use default
        // implementation of the fix_warnings() function)
        Box::new(key_without_value::KeyWithoutValueFixer::default()),
        Box::new(lowercase_key::LowercaseKeyFixer::default()),
        Box::new(space_character::SpaceCharacterFixer::default()),
        Box::new(trailing_whitespace::TrailingWhitespaceFixer::default()),
        Box::new(ending_blank_line::EndingBlankLineFixer::default()),
        // Then we should run the fixers that handle the line entry collection at whole.
        // And at the end we should run the fixer for ExtraBlankLine check (because the previous
        // fixers can create additional extra blank lines).
    ]
}

pub fn run(warnings: &mut [Warning], lines: &mut Vec<LineEntry>) -> usize {
    if warnings.is_empty() {
        return 0;
    }

    let mut count = 0;
    for fixer in fixlist() {
        // We can optimize it: create check_name:warnings map in advance
        let fixer_warnings: Vec<&mut Warning> = warnings
            .iter_mut()
            .filter(|w| w.check_name == fixer.name())
            .collect();

        if !fixer_warnings.is_empty() {
            match fixer.fix_warnings(fixer_warnings, lines) {
                Some(fixer_count) => count += fixer_count,
                None => {
                    for warning in warnings {
                        warning.mark_as_unfixed();
                    }
                    return 0;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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

    #[test]
    fn run_with_empty_warnings_test() {
        let mut lines = vec![line_entry(1, 2, "A=B"), blank_line_entry(2, 2)];
        let mut warnings: Vec<Warning> = Vec::new();

        assert_eq!(0, run(&mut warnings, &mut lines));
    }

    #[test]
    fn run_with_fixable_warning_test() {
        let mut lines = vec![
            line_entry(1, 3, "A=B"),
            line_entry(2, 3, "c=d"),
            blank_line_entry(3, 3),
        ];
        let mut warnings = vec![Warning::new(
            lines[1].clone(),
            "LowercaseKey",
            String::from("The c key should be in uppercase"),
        )];

        assert_eq!(1, run(&mut warnings, &mut lines));
        assert_eq!("C=d", lines[1].raw_string);
        assert!(warnings[0].is_fixed);
    }

    #[test]
    fn run_with_unfixable_warning_test() {
        let mut lines = vec![
            line_entry(1, 3, "A=B"),
            line_entry(2, 3, "UNFIXABLE-"),
            blank_line_entry(3, 3),
        ];
        let mut warnings = vec![Warning::new(
            lines[1].clone(),
            "Unfixable",
            String::from("The UNFIXABLE- key is not fixable"),
        )];

        assert_eq!(0, run(&mut warnings, &mut lines));
        assert!(!warnings[0].is_fixed);
    }

    #[test]
    fn run_when_lines_do_not_fit_numbers_test() {
        let mut lines = vec![
            line_entry(1, 3, "a=B"),
            line_entry(4, 3, "c=D"),
            blank_line_entry(3, 3),
        ];
        let mut warnings = vec![
            Warning::new(
                lines[0].clone(),
                "LowercaseKey",
                String::from("The a key should be in uppercase"),
            ),
            Warning::new(
                lines[1].clone(),
                "LowercaseKey",
                String::from("The c key should be in uppercase"),
            ),
        ];

        assert_eq!(0, run(&mut warnings, &mut lines));
        assert!(!warnings[0].is_fixed);
    }
}
