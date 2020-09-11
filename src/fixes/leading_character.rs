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
        let mut fixer = LeadingCharacterFixer::default();
        let mut lines = vec![
            line_entry(1, 7, ".FOO=BAR"),
            line_entry(2, 7, " Z=Y"),
            line_entry(3, 7, "*BAR=BAZ"),
            line_entry(4, 7, "1QUX=QUUX"),
            line_entry(5, 7, "_QUUX=FOOBAR"),
            line_entry(6, 7, "KEY=VALUE"),
            blank_line_entry(6, 7),
        ];

        let mut warnings = vec![
            Warning::new(
                lines[0].clone(),
                "LeadingCharacter",
                String::from("Invalid leading character detected"),
            ),
            Warning::new(
                lines[1].clone(),
                "LeadingCharacter",
                String::from("Invalid leading character detected"),
            ),
            Warning::new(
                lines[2].clone(),
                "LeadingCharacter",
                String::from("Invalid leading character detected"),
            ),
            Warning::new(
                lines[3].clone(),
                "LeadingCharacter",
                String::from("Invalid leading character detected"),
            ),
        ];

        assert_eq!(
            Some(4),
            fixer.fix_warnings(warnings.iter_mut().collect(), &mut lines)
        );

        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert_eq!("Z=Y", lines[1].raw_string);
        assert_eq!("BAR=BAZ", lines[2].raw_string);
        assert_eq!("QUX=QUUX", lines[3].raw_string);
        assert_eq!("_QUUX=FOOBAR", lines[4].raw_string);
        assert_eq!("KEY=VALUE", lines[5].raw_string);
        assert_eq!("\n", lines[6].raw_string);
    }
}
