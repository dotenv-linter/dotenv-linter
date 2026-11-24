use dotenv_core::{LineEntry, split_inline_comment};

use super::Fix;
use crate::LintKind;

#[derive(Default)]
pub(crate) struct ValueWithoutQuotesFixer {}

impl Fix for ValueWithoutQuotesFixer {
    fn name(&self) -> LintKind {
        LintKind::ValueWithoutQuotes
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let (value, comment) = split_inline_comment(line.get_value()?);
        let pure_val = format!("\"{value}\"");

        line.raw_string = match comment {
            Some(comment) => format!("{}={} {}", line.get_key()?, pure_val, comment),
            None => format!("{}={}", line.get_key()?, pure_val),
        };

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn fix_line_test() {
        let fixer = ValueWithoutQuotesFixer::default();
        let mut line = line_entry(1, 1, "FOO=bar baz");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=\"bar baz\"", line.raw_string);
    }

    #[test]
    fn fix_line_with_inline_comment_test() {
        let fixer = ValueWithoutQuotesFixer::default();

        for (input, expected) in [
            ("FOO=bar baz # comment", "FOO=\"bar baz\" # comment"),
            ("FOO=bar baz   # comment", "FOO=\"bar baz\" # comment"),
            (
                "FOO=bar baz # it's a comment",
                "FOO=\"bar baz\" # it's a comment",
            ),
            (
                "FOO=bar baz # comment # one more",
                "FOO=\"bar baz\" # comment # one more",
            ),
            (
                "FOO=тест мир # комментарий",
                "FOO=\"тест мир\" # комментарий",
            ),
            ("FOO=bar baz#qux", "FOO=\"bar baz#qux\""),
            ("FOO=bar baz \\# qux", "FOO=\"bar baz \\# qux\""),
        ] {
            let mut line = line_entry(1, 1, input);

            assert_eq!(Some(()), fixer.fix_line(&mut line), "input: {input}");
            assert_eq!(expected, line.raw_string, "input: {input}");
        }
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
