use super::Fix;
use crate::{checks::check_variants::Lint, common::*};

pub(crate) struct KeyWithoutValueFixer {}

impl Default for KeyWithoutValueFixer {
    fn default() -> Self {
        Self {}
    }
}

impl Fix for KeyWithoutValueFixer {
    fn name(&self) -> Lint {
        Lint::KeyWithoutValue
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
        let mut lines = vec![
            line_entry(1, 3, "FOO"),
            line_entry(2, 3, "Z=Y"),
            blank_line_entry(3, 3),
        ];
        let mut warning = Warning::new(
            lines[0].clone(),
            "KeyWithoutValue",
            "The FOO key should be with a value or have an equal sign",
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("FOO=", lines[0].raw_string);
    }
}
