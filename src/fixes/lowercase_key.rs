use super::Fix;
use crate::common::LintKind;
use dotenv_lookup::LineEntry;

#[derive(Default)]
pub(crate) struct LowercaseKeyFixer {}

impl Fix for LowercaseKeyFixer {
    fn name(&self) -> LintKind {
        LintKind::LowercaseKey
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;
        let key = key.to_uppercase();
        line.raw_string = format!("{}={}", key, line.get_value()?);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_line_test() {
        let fixer = LowercaseKeyFixer::default();
        let mut line = line_entry(1, 1, "foO=BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=BAR", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = LowercaseKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 3, "foO=BAR"),
            line_entry(2, 3, "Z=Y"),
            blank_line_entry(3, 3),
        ];
        let warning_lines = [lines[0].number];

        assert_eq!(Some(1), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("FOO=BAR", lines[0].raw_string);
    }
}
