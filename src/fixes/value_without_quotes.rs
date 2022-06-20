use super::Fix;
use crate::common::{LineEntry, LintKind};

#[derive(Default)]
pub(crate) struct ValueWithoutQuotesFixer {}

impl Fix for ValueWithoutQuotesFixer {
    fn name(&self) -> LintKind {
        LintKind::ValueWithoutQuotes
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let pure_val = format!("\"{}\"", line.get_value()?);

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
        let fixer = ValueWithoutQuotesFixer::default();
        let mut line = line_entry(1, 1, "FOO=bar baz");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=\"bar baz\"", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = ValueWithoutQuotesFixer::default();
        let mut lines = vec![
            line_entry(1, 3, "FOO=bar baz"),
            line_entry(2, 3, "Z=\"Y X\""),
            blank_line_entry(3, 3),
        ];
        let warning_lines = [lines[0].number];

        assert_eq!(Some(1), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("FOO=\"bar baz\"", lines[0].raw_string);
    }
}
