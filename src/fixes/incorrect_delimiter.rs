use super::Fix;
use crate::common::*;

pub(crate) struct IncorrectDelimiterFixer<'a> {
    name: &'a str,
}

impl Default for IncorrectDelimiterFixer<'_> {
    fn default() -> Self {
        Self {
            name: "IncorrectDelimiter",
        }
    }
}

impl Fix for IncorrectDelimiterFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;

        let cleaned_key = remove_invalid_leading_chars(&key);
        let start_idx = key.len() - cleaned_key.len();

        let cleaned_key = key[start_idx..].replace(|c: char| !c.is_alphanumeric(), "_");

        line.raw_string = format!("{}{}={}", &key[..start_idx], cleaned_key, line.get_value()?);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_line_test() {
        let mut fixer = IncorrectDelimiterFixer::default();
        let mut line = line_entry(1, 1, "RAILS-ENV=development");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("RAILS_ENV=development", line.raw_string);
    }

    #[test]
    fn fix_line_with_invalid_prefix_test() {
        let mut fixer = IncorrectDelimiterFixer::default();
        let mut line = line_entry(1, 1, "**RAILS-ENV=development");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("**RAILS_ENV=development", line.raw_string);
    }

    #[test]
    fn fix_line_with_invalid_suffix_test() {
        let mut fixer = IncorrectDelimiterFixer::default();
        let mut line = line_entry(1, 1, "RAILS-ENV--=development");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("RAILS_ENV__=development", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut IncorrectDelimiterFixer::default(),
            vec![
                TestLine::new("RAILS-ENV=development").warning(
                    "IncorrectDelimiter",
                    "The RAILS-ENV key has has an incorrect delimter",
                ),
                TestLine::new("RAILS_ENV=true"),
                TestLine::new("\n"),
            ]
            .into(),
        );

        assert_eq!(Some(1), fix_count);
        assert_eq!(
            vec!["RAILS_ENV=development", "RAILS_ENV=true", "\n"],
            fixed_lines
        );
    }
}
