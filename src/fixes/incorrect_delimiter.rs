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

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;

        let cleaned_key = remove_invalid_leading_chars(&key);
        let start_idx = key.len() - cleaned_key.len();

        let cleaned_key: String = key
            .chars()
            .skip(start_idx)
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();

        let fixed_key = key[..start_idx].to_string() + &cleaned_key;

        line.raw_string = format!("{}={}", fixed_key, line.get_value()?);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

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
        let mut warning = Warning::new(
            lines[0].clone(),
            "IncorrectDelimiter",
            String::from("The RAILS-ENV key has has an incorrect delimter"),
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("RAILS_ENV=development", lines[0].raw_string);
        assert!(warning.is_fixed);
    }
}
