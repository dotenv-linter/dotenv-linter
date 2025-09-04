use dotenv_core::LineEntry;

use super::Fix;
use crate::LintKind;

#[derive(Default)]
pub(crate) struct SpaceCharacterFixer {}

impl Fix for SpaceCharacterFixer {
    fn name(&self) -> LintKind {
        LintKind::SpaceCharacter
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;
        let value = line.get_value()?;
        line.raw_string = format!("{}={}", key.trim_end(), value.trim_start());

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn fix_line_test() {
        let fixer = SpaceCharacterFixer::default();
        let mut line = line_entry(1, 1, "FOO = BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=BAR", line.raw_string);
    }

    #[test]
    fn trailing_should_not_be_fixed() {
        let fixer = SpaceCharacterFixer::default();
        let mut line = line_entry(1, 1, "DEBUG_HTTP=true ");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("DEBUG_HTTP=true ", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = SpaceCharacterFixer::default();
        let mut lines = vec![
            line_entry(1, 3, "FOO= BAR"),
            line_entry(2, 3, "Z =Y"),
            blank_line_entry(3, 3),
        ];
        let warning_lines = [lines[0].number, lines[1].number];

        assert_eq!(Some(2), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert_eq!("Z=Y", lines[1].raw_string);
    }
}
