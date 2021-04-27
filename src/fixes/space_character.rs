use super::Fix;
use crate::common::*;

pub(crate) struct SpaceCharacterFixer<'a> {
    name: &'a str,
}

impl Default for SpaceCharacterFixer<'_> {
    fn default() -> Self {
        Self {
            name: "SpaceCharacter",
        }
    }
}

impl Fix for SpaceCharacterFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;
        let value = line.get_value()?;
        line.raw_string = format!("{}={}", key.trim_end(), value.trim_start());

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
    fn fix_line_test() {
        let mut fixer = SpaceCharacterFixer::default();
        let mut line = line_entry(1, 1, "FOO = BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=BAR", line.raw_string);
    }

    #[test]
    fn trailing_should_not_be_fixed() {
        let mut fixer = SpaceCharacterFixer::default();
        let mut line = line_entry(1, 1, "DEBUG_HTTP=true ");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("DEBUG_HTTP=true ", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let mut fixer = SpaceCharacterFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "FOO= BAR" => Some(("SpaceCharacter","The line has spaces around equal sign")),
            "Z =Y" => Some(("SpaceCharacter","The line has spaces around equal sign")),
            "" => None,
        ];
        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(2), fix_count);
        assert_eq!(vec!["FOO=BAR", "Z=Y", ""], fixed_lines);
    }
}
