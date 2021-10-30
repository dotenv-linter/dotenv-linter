use crate::common::{LineEntry, LintKind, Warning};

mod duplicated_key;
mod ending_blank_line;
mod extra_blank_line;
mod incorrect_delimiter;
mod key_without_value;
mod leading_character;
mod lowercase_key;
mod quote_character;
mod space_character;
mod substitution_key;
mod trailing_whitespace;
mod unordered_key;

trait Fix {
    fn name(&self) -> LintKind;

    fn fix_warnings(&self, warning_lines: &[usize], lines: &mut Vec<LineEntry>) -> Option<usize> {
        let mut count: usize = 0;
        for line_number in warning_lines {
            let line = lines.get_mut(line_number - 1)?;
            if self.fix_line(line).is_some() {
                count += 1;
            }
        }

        Some(count)
    }

    fn fix_line(&self, _line: &mut LineEntry) -> Option<()> {
        None
    }

    fn is_mandatory(&self) -> bool {
        false
    }
}

// Fix order matters
fn fixlist() -> Vec<Box<dyn Fix>> {
    vec![
        // At first we run the fixers that handle a single line entry (they use default
        // implementation of the fix_warnings() function)
        Box::new(key_without_value::KeyWithoutValueFixer::default()),
        Box::new(lowercase_key::LowercaseKeyFixer::default()),
        Box::new(space_character::SpaceCharacterFixer::default()),
        Box::new(trailing_whitespace::TrailingWhitespaceFixer::default()),
        Box::new(leading_character::LeadingCharacterFixer::default()),
        Box::new(quote_character::QuoteCharacterFixer::default()),
        Box::new(incorrect_delimiter::IncorrectDelimiterFixer::default()),
        Box::new(extra_blank_line::ExtraBlankLineFixer::default()),
        Box::new(substitution_key::SubstitutionKeyFixer::default()),
        // Then we should run the fixers that handle the line entry collection at whole
        Box::new(unordered_key::UnorderedKeyFixer::default()),
        Box::new(duplicated_key::DuplicatedKeyFixer::default()),
        Box::new(ending_blank_line::EndingBlankLineFixer::default()),
    ]
}

pub fn run(warnings: &[Warning], lines: &mut Vec<LineEntry>, skip_checks: &[LintKind]) -> usize {
    if warnings.is_empty() {
        return 0;
    }
    let mut fixes = fixlist();

    // Skip fixes for checks in --skip argument (globally)
    fixes.retain(|f| !skip_checks.contains(&f.name()));

    let mut count = 0;
    for fixer in fixes {
        // We can optimize it: create check_name:warnings map in advance
        let warning_lines: Vec<usize> = warnings
            .iter()
            .filter(|w| w.check_name == fixer.name())
            .map(|w| w.line_number)
            .collect();

        // Some fixers are mandatory because previous fixers can spawn warnings for them
        if fixer.is_mandatory() || !warning_lines.is_empty() {
            match fixer.fix_warnings(&warning_lines, lines) {
                Some(fixer_count) => count += fixer_count,
                None => {
                    return 0;
                }
            }
        }
    }

    // Removes extra blank lines
    lines.retain(|l| !l.is_deleted);

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn run_with_empty_warnings_test() {
        let mut lines = vec![line_entry(1, 2, "A=B"), blank_line_entry(2, 2)];
        let mut warnings: Vec<Warning> = Vec::new();

        assert_eq!(0, run(&mut warnings, &mut lines, &[]));
    }

    #[test]
    fn run_with_fixable_warning_test() {
        let mut lines = vec![
            line_entry(1, 3, "A=B"),
            line_entry(2, 3, "c=d"),
            blank_line_entry(3, 3),
        ];
        let mut warnings = vec![Warning::new(
            lines[1].number,
            LintKind::LowercaseKey,
            "The c key should be in uppercase",
        )];

        assert_eq!(1, run(&mut warnings, &mut lines, &[]));
        assert_eq!("C=d", lines[1].raw_string);
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
                lines[0].number,
                LintKind::LowercaseKey,
                "The a key should be in uppercase",
            ),
            Warning::new(
                lines[1].number,
                LintKind::LowercaseKey,
                "The c key should be in uppercase",
            ),
        ];

        assert_eq!(0, run(&mut warnings, &mut lines, &[]));
    }

    #[test]
    fn new_warnings_after_fix_test() {
        let mut lines = vec![
            line_entry(1, 5, "A1=1"),
            line_entry(2, 5, "A2=2"),
            line_entry(3, 5, "a0=0"),
            line_entry(4, 5, "a2=2"),
            blank_line_entry(5, 5),
        ];
        let warnings = vec![
            Warning::new(
                lines[2].number,
                LintKind::LowercaseKey,
                "The a0 key should be in uppercase",
            ),
            Warning::new(
                lines[3].number,
                LintKind::LowercaseKey,
                "The a2 key should be in uppercase",
            ),
        ];

        assert_eq!(2, run(&warnings, &mut lines, &[]));
        assert_eq!("A0=0", lines[0].raw_string);
        assert_eq!("A1=1", lines[1].raw_string);
        assert_eq!("A2=2", lines[2].raw_string);
        assert_eq!("# A2=2", lines[3].raw_string);
        assert_eq!("\n", lines[4].raw_string);
    }

    #[test]
    fn skip_duplicated_key() {
        let mut lines = vec![
            line_entry(1, 5, "A1=1"),
            line_entry(2, 5, "A2=2"),
            line_entry(3, 5, "a0=0"),
            line_entry(4, 5, "a2=2"),
            blank_line_entry(5, 5),
        ];
        let warnings = vec![
            Warning::new(
                lines[2].number,
                LintKind::LowercaseKey,
                "The a0 key should be in uppercase",
            ),
            Warning::new(
                lines[3].number,
                LintKind::LowercaseKey,
                "The a2 key should be in uppercase",
            ),
        ];

        assert_eq!(2, run(&warnings, &mut lines, &[LintKind::DuplicatedKey]));
        assert_eq!("A0=0", lines[0].raw_string);
        assert_eq!("A1=1", lines[1].raw_string);
        assert_eq!("A2=2", lines[2].raw_string);
        assert_eq!("A2=2", lines[3].raw_string);
        assert_eq!("\n", lines[4].raw_string);
    }

    #[test]
    fn skip_unordered_key() {
        let mut lines = vec![
            line_entry(1, 5, "A1=1"),
            line_entry(2, 5, "A2=2"),
            line_entry(3, 5, "a0=0"),
            line_entry(4, 5, "a2=2"),
            blank_line_entry(5, 5),
        ];
        let warnings = vec![
            Warning::new(
                lines[2].number,
                LintKind::LowercaseKey,
                "The a0 key should be in uppercase",
            ),
            Warning::new(
                lines[3].number,
                LintKind::LowercaseKey,
                "The a2 key should be in uppercase",
            ),
        ];

        assert_eq!(2, run(&warnings, &mut lines, &[LintKind::UnorderedKey]));
        assert_eq!("A1=1", lines[0].raw_string);
        assert_eq!("A2=2", lines[1].raw_string);
        assert_eq!("A0=0", lines[2].raw_string);
        assert_eq!("# A2=2", lines[3].raw_string);
        assert_eq!("\n", lines[4].raw_string);
    }

    struct TestFixer {
        name: LintKind,
    }

    impl Fix for TestFixer {
        fn name(&self) -> LintKind {
            self.name
        }

        fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
            if line.raw_string.chars().count() > 5 {
                Some(())
            } else {
                None
            }
        }
    }
}
