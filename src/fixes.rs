use crate::common::*;

mod ending_blank_line;
mod extra_blank_line;
mod lowercase_key;
mod unordered_key;

trait Fix {
    fn name(&self) -> &str;

    fn fix_warnings(
        &self,
        warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        let mut count: usize = 0;
        for warning in warnings {
            let line = &mut lines[warning.line_number() - 1];
            if self.fix_line(line).is_some() {
                warning.set_fixed(true);
                count += 1;
            }
        }

        Some(count)
    }

    fn fix_line(&self, _line: &mut LineEntry) -> Option<()> {
        None
    }

    fn set_fixed_on_all(&self, warnings: Vec<&mut Warning>) -> Option<usize> {
        let count = warnings.len();
        for warning in warnings {
            warning.set_fixed(true);
        }

        Some(count)
    }
}

// TODO: skip fixes (like checks)
// The fix order is matter
fn fixlist() -> Vec<Box<dyn Fix>> {
    vec![
        // At first we run the fixers that handle a single line entry (they use default
        // implementation of the fix_warnings() function)
        Box::new(lowercase_key::LowercaseKeyFixer::default()),
        // Then we run the fixers that handle the line entry collection at whole
        Box::new(ending_blank_line::EndingBlankLineFixer::default()),
        Box::new(unordered_key::UnorderedKeyFixer::default()),
        // At the end we run ExtraBlankLineFixer (because the previous fixers can create
        // additional extra blank lines)
        Box::new(extra_blank_line::ExtraBlankLineFixer::default()),
    ]
}

pub fn run(warnings: &mut [Warning], lines: &mut Vec<LineEntry>) -> usize {
    if warnings.is_empty() {
        return 0;
    }

    for warning in warnings.iter_mut() {
        warning.set_fixed(false);
    }

    let mut count = 0;
    for fixer in fixlist() {
        // We can optimize it: create check_name:warnings map in advance
        let fixer_warnings: Vec<&mut Warning> = warnings
            .iter_mut()
            .filter(|w| w.check_name == fixer.name())
            .collect();
        if !fixer_warnings.is_empty() || fixer.name() == "ExtraBlankLine" {
            count += fixer.fix_warnings(fixer_warnings, lines).unwrap_or(0);
        }
    }

    count
}
