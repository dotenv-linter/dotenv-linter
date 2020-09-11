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

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        line.raw_string.push('=');

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_line_test() {
        let fixer = KeyWithoutValueFixer::default();
        let mut line = line_entry(1, 1, "FOO");

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = KeyWithoutValueFixer::default();
        let mut lines = vec![
            line_entry(1, 3, "FOO"),
            line_entry(2, 3, "Z=Y"),
            blank_line_entry(3, 3),
        ];
        let mut warning = Warning::new(
            lines[0].clone(),
            "KeyWithoutValue",
            String::from("The FOO key should be with a value or have an equal sign"),
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("FOO=", lines[0].raw_string);
        assert!(warning.is_fixed);
    }
}
