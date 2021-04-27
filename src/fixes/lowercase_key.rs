use super::Fix;
use crate::common::*;

pub(crate) struct LowercaseKeyFixer<'a> {
    name: &'a str,
}

impl Default for LowercaseKeyFixer<'_> {
    fn default() -> Self {
        Self {
            name: "LowercaseKey",
        }
    }
}

impl Fix for LowercaseKeyFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
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
    use crate::fixes::run_fix_warnings;
    use crate::lines_and_warnings;

    #[test]
    fn fix_line_test() {
        let mut fixer = LowercaseKeyFixer::default();
        let mut line = line_entry(1, 1, "foO=BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=BAR", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let mut fixer = LowercaseKeyFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "foO=BAR" => Some(("LowercaseKey","The FOO key should be in uppercase")),
            "Z=Y" => None,
            "" => None,
        ];
        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(1), fix_count);
        assert_eq!(vec!["FOO=BAR", "Z=Y", ""], fixed_lines);
    }
}
