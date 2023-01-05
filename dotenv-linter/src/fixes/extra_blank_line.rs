use super::Fix;
use crate::common::LintKind;
use dotenv_lookup::LineEntry;

#[derive(Default)]
pub(crate) struct ExtraBlankLineFixer {}

impl Fix for ExtraBlankLineFixer {
    fn name(&self) -> LintKind {
        LintKind::ExtraBlankLine
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        line.mark_as_deleted();
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::line_entry;

    #[test]
    fn no_blank_lines_test() {
        let fixer = ExtraBlankLineFixer::default();
        let mut lines = vec![
            line_entry(1, 3, "FOO=BAR"),
            line_entry(2, 3, ""),
            line_entry(3, 3, "HOGE=HUGA"),
        ];
        let warning_lines = [];

        assert_eq!(Some(0), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!(lines, lines);
    }

    #[test]
    fn fix_one_extra_blank_line_test() {
        let fixer = ExtraBlankLineFixer::default();
        let mut lines = vec![
            line_entry(1, 4, "FOO=BAR"),
            line_entry(2, 4, ""),
            line_entry(3, 4, ""),
            line_entry(4, 4, "HOGE=HUGA"),
        ];
        let warning_lines = [lines[2].number];

        assert_eq!(Some(1), fixer.fix_warnings(&warning_lines, &mut lines));
    }

    #[test]
    fn fix_multiple_blank_lines_test() {
        let fixer = ExtraBlankLineFixer::default();
        let mut lines = vec![
            line_entry(1, 5, "FOO=BAR"),
            line_entry(2, 5, ""),
            line_entry(3, 5, ""),
            line_entry(4, 5, ""),
            line_entry(5, 5, "HOGE=HUGA"),
        ];
        let warning_lines = [lines[2].number, lines[3].number];

        assert_eq!(Some(2), fixer.fix_warnings(&warning_lines, &mut lines));
    }
}
