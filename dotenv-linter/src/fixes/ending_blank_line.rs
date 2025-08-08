use dotenv_lookup::LineEntry;

use super::Fix;
use crate::common::{LintKind, LF};

#[derive(Default)]
pub(crate) struct EndingBlankLineFixer {}

impl Fix for EndingBlankLineFixer {
    fn name(&self) -> LintKind {
        LintKind::EndingBlankLine
    }

    fn fix_warnings(&self, _: &[usize], lines: &mut Vec<LineEntry>) -> Option<usize> {
        let last_line = lines.last()?;

        if last_line.raw_string.ends_with(LF) {
            return Some(0);
        }

        lines.push(LineEntry::new(lines.len() + 1, LF, true));

        Some(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::line_entry;

    #[test]
    fn fix_warnings_test() {
        let fixer = EndingBlankLineFixer::default();
        let mut lines = vec![line_entry(1, 2, "FOO=BAR"), line_entry(2, 2, "Z=Y")];
        let warning_lines = [lines[1].number];

        assert_eq!(Some(1), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("\n", lines[2].raw_string);
    }

    #[test]
    fn ending_blank_line_exist_test() {
        let fixer = EndingBlankLineFixer::default();
        let mut lines = vec![line_entry(1, 2, "FOO=BAR"), line_entry(2, 2, LF)];

        assert_eq!(Some(0), fixer.fix_warnings(&[], &mut lines));
        assert_eq!(lines.len(), 2);
    }
}
