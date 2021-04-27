use super::Fix;
use crate::common::*;

pub(crate) struct KeyWithoutValueFixer<'a> {
    name: &'a str,
}

impl Default for KeyWithoutValueFixer<'_> {
    fn default() -> Self {
        Self {
            name: "KeyWithoutValue",
        }
    }
}

impl Fix for KeyWithoutValueFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        line.raw_string.push('=');

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
        let mut fixer = KeyWithoutValueFixer::default();
        let mut line = line_entry(1, 1, "FOO");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let mut fixer = KeyWithoutValueFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "FOO" => Some(("KeyWithoutValue","The FOO key should be with a value or have an equal sign")),
            "Z=Y" => None,
            "\n" => None,
        ];

        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(1), fix_count);
        assert_eq!(vec!["FOO=", "Z=Y", "\n"], fixed_lines);
    }
}
