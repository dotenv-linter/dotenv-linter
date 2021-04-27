use super::Fix;
use crate::common::*;

pub(crate) struct ExtraBlankLineFixer<'a> {
    name: &'a str,
}

impl Default for ExtraBlankLineFixer<'_> {
    fn default() -> Self {
        Self {
            name: "ExtraBlankLine",
        }
    }
}

impl Fix for ExtraBlankLineFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        line.mark_as_deleted();
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;
    use crate::fixes::run_fix_warnings;
    use crate::lines_and_warnings;

    #[test]
    fn no_blank_lines_test() {
        let mut fixer = ExtraBlankLineFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "FOO=BAR" => None,
            "" => None,
            "HOGE=HUGA" => None,
        ];

        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(0), fix_count);
        assert_eq!(vec!["FOO=BAR", "", "HOGE=HUGA"], fixed_lines);
    }

    #[test]
    fn fix_one_extra_blank_line_test() {
        let mut fixer = ExtraBlankLineFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "FOO=BAR" => None,
            "" => None,
            "" => Some(("ExtraBlankLine", "Extra blank line detected")),
            "HOGE=HUGA" => None,
        ];

        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(1), fix_count);
        assert_eq!(vec!["FOO=BAR", "", "HOGE=HUGA"], fixed_lines);
    }

    #[test]
    fn fix_multiple_blank_lines_test() {
        let mut fixer = ExtraBlankLineFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "FOO=BAR" => None,
            "" => None,
            "" => Some(("ExtraBlankLine", "Extra blank line detected")),
            "" => Some(("ExtraBlankLine", "Extra blank line detected")),
            "HOGE=HUGA" => None,
        ];

        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(2), fix_count);
        assert_eq!(vec!["FOO=BAR", "", "HOGE=HUGA"], fixed_lines);
    }
}
