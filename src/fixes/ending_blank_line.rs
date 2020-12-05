use super::Fix;
use crate::common::*;

pub(crate) struct EndingBlankLineFixer<'a> {
    name: &'a str,
}

impl Default for EndingBlankLineFixer<'_> {
    fn default() -> Self {
        Self {
            name: "EndingBlankLine",
        }
    }
}

impl Fix for EndingBlankLineFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_warnings(
        &mut self,
        _warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        let file = lines.first()?.file.clone();
        let last_line = lines.last()?;

        if last_line.raw_string.ends_with(LF) {
            return Some(0);
        }

        lines.push(LineEntry {
            number: lines.len() + 1,
            file,
            raw_string: LF.to_string(),
            is_deleted: false,
        });

        Some(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_warnings_test() {
        let mut fixer = EndingBlankLineFixer::default();
        let mut lines = vec![line_entry(1, 2, "FOO=BAR"), line_entry(2, 2, "Z=Y")];
        let mut warning = Warning::new(
            lines[1].clone(),
            "EndingBlankLine",
            "No blank line at the end of the file",
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("\n", lines[2].raw_string);
    }

    #[test]
    fn ending_blank_line_exist_test() {
        let mut fixer = EndingBlankLineFixer::default();
        let mut lines = vec![line_entry(1, 2, "FOO=BAR"), line_entry(2, 2, LF)];

        assert_eq!(Some(0), fixer.fix_warnings(vec![], &mut lines));
        assert_eq!(lines.len(), 2);
    }
}
