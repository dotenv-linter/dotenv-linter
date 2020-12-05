use super::Fix;
use crate::common::*;

pub(crate) struct QuoteCharacterFixer<'a> {
    name: &'a str,
}

impl Default for QuoteCharacterFixer<'_> {
    fn default() -> Self {
        Self {
            name: "QuoteCharacter",
        }
    }
}

impl Fix for QuoteCharacterFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
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
        let mut fixer = QuoteCharacterFixer::default();
        let mut line = line_entry(1, 1, "FOO=\'\"b\'\"ar\"\'");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=bar", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let mut fixer = QuoteCharacterFixer::default();
        let mut lines = vec![
            line_entry(1, 3, "FOO=\"bar\'\""),
            line_entry(2, 3, "Z=Y"),
            blank_line_entry(3, 3),
        ];
        let mut warning = Warning::new(
            lines[0].clone(),
            "QuoteCharacter",
            "The value has quote characters (\', \")",
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("FOO=bar", lines[0].raw_string);
    }
}
