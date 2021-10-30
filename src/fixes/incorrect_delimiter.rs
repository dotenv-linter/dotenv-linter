use super::Fix;
use crate::common::{remove_invalid_leading_chars, LineEntry, LintKind};

pub(crate) struct IncorrectDelimiterFixer {}

impl Default for IncorrectDelimiterFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for IncorrectDelimiterFixer {
    fn name(&self) -> LintKind {
        LintKind::IncorrectDelimiter
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;

        let cleaned_key = remove_invalid_leading_chars(key);
        let start_idx = key.len() - cleaned_key.len();

        let cleaned_key = key[start_idx..].replace(|c: char| !c.is_alphanumeric(), "_");

        line.raw_string = format!("{}{}={}", &key[..start_idx], cleaned_key, line.get_value()?);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::{blank_line_entry, line_entry};

    #[test]
    fn fix_line_test() {
        let fixer = IncorrectDelimiterFixer::default();
        let mut line = line_entry(1, 1, "RAILS-ENV=development");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("RAILS_ENV=development", line.raw_string);
    }

    #[test]
    fn fix_line_with_invalid_prefix_test() {
        let fixer = IncorrectDelimiterFixer::default();
        let mut line = line_entry(1, 1, "**RAILS-ENV=development");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("**RAILS_ENV=development", line.raw_string);
    }

    #[test]
    fn fix_line_with_invalid_suffix_test() {
        let fixer = IncorrectDelimiterFixer::default();
        let mut line = line_entry(1, 1, "RAILS-ENV--=development");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("RAILS_ENV__=development", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = IncorrectDelimiterFixer::default();
        let mut lines = vec![
            line_entry(1, 3, "RAILS-ENV=development"),
            line_entry(2, 3, "RAILS_ENV=true"),
            blank_line_entry(3, 3),
        ];
        let warning_lines = [lines[0].number];

        assert_eq!(Some(1), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("RAILS_ENV=development", lines[0].raw_string);
    }
}
