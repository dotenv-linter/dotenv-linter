use super::Fix;
use crate::common::*;

pub(crate) struct LeadingCharacterFixer<'a> {
    name: &'a str,
}

impl Default for LeadingCharacterFixer<'_> {
    fn default() -> Self {
        Self {
            name: "LeadingCharacter",
        }
    }
}

impl Fix for LeadingCharacterFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;

        let cleaned_key = remove_invalid_leading_chars(&key);

        line.raw_string = format!("{}={}", cleaned_key, line.get_value()?);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_leading_dot() {
        let mut fixer = LeadingCharacterFixer::default();
        let mut leading_period = line_entry(1, 1, ".FOO=BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut leading_period));
        assert_eq!("FOO=BAR", leading_period.raw_string);
    }

    #[test]
    fn fix_leading_space() {
        let mut fixer = LeadingCharacterFixer::default();
        let mut leading_space = line_entry(1, 1, " FOO=BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut leading_space));
        assert_eq!("FOO=BAR", leading_space.raw_string);
    }

    #[test]
    fn fix_leading_asterisk() {
        let mut fixer = LeadingCharacterFixer::default();
        let mut leading_asterisk = line_entry(1, 1, "*FOO=BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut leading_asterisk));
        assert_eq!("FOO=BAR", leading_asterisk.raw_string);
    }

    #[test]
    fn fix_leading_number() {
        let mut fixer = LeadingCharacterFixer::default();
        let mut leading_number = line_entry(1, 1, "1FOO=BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut leading_number));
        assert_eq!("FOO=BAR", leading_number.raw_string);
    }

    #[test]
    fn fix_many_invalid_leading_chars() {
        let mut fixer = LeadingCharacterFixer::default();
        let mut leading_number = line_entry(1, 1, "-1&*FOO=BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut leading_number));
        assert_eq!("FOO=BAR", leading_number.raw_string);
    }

    #[test]
    fn leading_underscore_is_unchanged() {
        let mut fixer = LeadingCharacterFixer::default();
        let mut leading_underscore = line_entry(1, 1, "_FOO=BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut leading_underscore));
        assert_eq!("_FOO=BAR", leading_underscore.raw_string);
    }

    #[test]
    fn no_leading_char_is_unchanged() {
        let mut fixer = LeadingCharacterFixer::default();
        let mut normal = line_entry(1, 1, "FOO=BAR");

        assert_eq!(Some(()), fixer.fix_line(&mut normal));
        assert_eq!("FOO=BAR", normal.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let warning_name = "LeadingCharacter";
        let message = "Invalid leading character detected";

        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut LeadingCharacterFixer::default(),
            vec![
                TestLine::new(".FOO=BAR").warning(warning_name, message),
                TestLine::new(" Z=Y").warning(warning_name, message),
                TestLine::new("*BAR=BAZ").warning(warning_name, message),
                TestLine::new("1QUX=QUUX").warning(warning_name, message),
                TestLine::new("_QUUX=FOOBAR"),
                TestLine::new("KEY=VALUE"),
                TestLine::new("\n"),
            ]
            .into(),
        );

        assert_eq!(Some(4), fix_count);
        assert_eq!(
            vec![
                "FOO=BAR",
                "Z=Y",
                "BAR=BAZ",
                "QUX=QUUX",
                "_QUUX=FOOBAR",
                "KEY=VALUE",
                "\n",
            ],
            fixed_lines
        );
    }
}
