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
        let (fix_count, fixed_lines) = run_fix_warnings(
            &mut KeyWithoutValueFixer::default(),
            vec![
                TestLine::new("FOO").warning(
                    "KeyWithoutValue",
                    "The FOO key should be with a value or have an equal sign",
                ),
                TestLine::new("Z=Y"),
                TestLine::new("\n"),
            ]
            .into(),
        );

        assert_eq!(Some(1), fix_count);
        assert_eq!(vec!["FOO=", "Z=Y", "\n"], fixed_lines);
    }
}
