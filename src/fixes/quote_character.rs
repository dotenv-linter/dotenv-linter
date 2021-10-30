use super::Fix;
use crate::common::{LineEntry, LintKind};

pub(crate) struct QuoteCharacterFixer {}

impl Default for QuoteCharacterFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for QuoteCharacterFixer {
    fn name(&self) -> LintKind {
        LintKind::QuoteCharacter
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let value = line.get_value()?;
        let pure_val = value.replace("'", "").replace("\"", "");

        line.raw_string = format!("{}={}", line.get_key()?, pure_val);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_line_test() {
        let fixer = QuoteCharacterFixer::default();
        let mut line = line_entry(1, 1, "FOO=\'\"b\'\"ar\"\'");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=bar", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = QuoteCharacterFixer::default();
        let mut lines = vec![
            line_entry(1, 3, "FOO=\"bar\'\""),
            line_entry(2, 3, "Z=Y"),
            blank_line_entry(3, 3),
        ];
        let warning_lines = [lines[0].number];

        assert_eq!(Some(1), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("FOO=bar", lines[0].raw_string);
    }
}
