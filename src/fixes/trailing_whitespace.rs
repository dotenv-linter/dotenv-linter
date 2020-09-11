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
            "TrailingWhitespace",
            String::from("Trailing whitespace detected"),
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert!(warning.is_fixed);
    }
}
