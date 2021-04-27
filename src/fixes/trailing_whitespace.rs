use super::Fix;
use crate::common::*;

pub(crate) struct TrailingWhitespaceFixer<'a> {
    name: &'a str,
}

impl Default for TrailingWhitespaceFixer<'_> {
    fn default() -> Self {
        Self {
            name: "TrailingWhitespace",
        }
    }
}

impl Fix for TrailingWhitespaceFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        line.raw_string = line.raw_string.trim_end().to_string();

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
        let mut fixer = TrailingWhitespaceFixer::default();
        let mut line = line_entry(1, 1, "DEBUG_HTTP=true  ");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("DEBUG_HTTP=true", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let mut fixer = TrailingWhitespaceFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "FOO=BAR " => Some(("TrailingWhitespace","Trailing whitespace detected")),
            "Z=Y" => None,
            "" => None,
        ];
        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(1), fix_count);
        assert_eq!(vec!["FOO=BAR", "Z=Y", ""], fixed_lines);
    }
}
