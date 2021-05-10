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

    #[test]
    fn no_blank_lines_test() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut ExtraBlankLineFixer::default(),
            vec![
                TestLine::new("FOO=BAR"),
                TestLine::new(""),
                TestLine::new("HOGE=HUGA"),
            ]
            .into(),
        );

        assert_eq!(Some(0), fix_count);
        assert_eq!(vec!["FOO=BAR", "", "HOGE=HUGA"], fixed_lines);
    }

    #[test]
    fn fix_one_extra_blank_line_test() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut ExtraBlankLineFixer::default(),
            vec![
                TestLine::new("FOO=BAR"),
                TestLine::new(""),
                TestLine::new("").warning("ExtraBlankLine", "Extra blank line detected"),
                TestLine::new("HOGE=HUGA"),
            ]
            .into(),
        );

        assert_eq!(Some(1), fix_count);
        assert_eq!(vec!["FOO=BAR", "", "HOGE=HUGA"], fixed_lines);
    }

    #[test]
    fn fix_multiple_blank_lines_test() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut ExtraBlankLineFixer::default(),
            vec![
                TestLine::new("FOO=BAR"),
                TestLine::new(""),
                TestLine::new("").warning("ExtraBlankLine", "Extra blank line detected"),
                TestLine::new("").warning("ExtraBlankLine", "Extra blank line detected"),
                TestLine::new("HOGE=HUGA"),
            ]
            .into(),
        );

        assert_eq!(Some(2), fix_count);
        assert_eq!(vec!["FOO=BAR", "", "HOGE=HUGA"], fixed_lines);
    }
}
