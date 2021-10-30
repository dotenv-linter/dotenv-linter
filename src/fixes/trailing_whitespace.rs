use super::Fix;
use crate::common::{LineEntry, LintKind};

pub(crate) struct TrailingWhitespaceFixer {}

impl Default for TrailingWhitespaceFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for TrailingWhitespaceFixer {
    fn name(&self) -> LintKind {
        LintKind::TrailingWhitespace
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        line.raw_string = line.raw_string.trim_end().to_string();

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{tests::*, Warning};

    #[test]
    fn fix_line_test() {
        let mut fixer = TrailingWhitespaceFixer::default();
        let mut line = line_entry(1, 1, "DEBUG_HTTP=true  ");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("DEBUG_HTTP=true", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let mut fixer = TrailingWhitespaceFixer::default();
        let mut lines = vec![
            line_entry(1, 3, "FOO=BAR "),
            line_entry(2, 3, "Z=Y"),
            blank_line_entry(3, 3),
        ];
        let mut warning = Warning::new(
            lines[0].clone(),
            LintKind::TrailingWhitespace,
            "Trailing whitespace detected",
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("FOO=BAR", lines[0].raw_string);
    }
}
