use super::Fix;
use crate::common::*;
use std::collections::HashSet;

pub(crate) struct DuplicatedKeyFixer<'a> {
    name: &'a str,
}

impl Default for DuplicatedKeyFixer<'_> {
    fn default() -> Self {
        Self {
            name: "DuplicatedKey",
        }
    }
}

impl Fix for DuplicatedKeyFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_warnings(
        &mut self,
        warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        let mut keys = HashSet::with_capacity(lines.len());
        let mut is_disabled = false;

        for line in lines {
            if let Some(comment) = line.get_control_comment() {
                if comment.checks.contains(&self.name) {
                    is_disabled = comment.is_disabled();
                }
            }
            if is_disabled {
                continue;
            }

            if let Some(key) = line.get_key() {
                if keys.contains(key) {
                    self.fix_line(line)?;
                } else {
                    keys.insert(key.to_string());
                }
            }
        }

        Some(warnings.len())
    }

    fn fix_line(&mut self, line: &mut LineEntry) -> Option<()> {
        line.raw_string = format!("# {}", line.raw_string);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn fix_warnings() {
        let mut fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 4, "FOO=BAR"),
            line_entry(2, 4, "Z=Y"),
            line_entry(3, 4, "FOO=BAZ"),
            line_entry(4, 4, "Z=X"),
        ];
        let mut warnings = vec![
            Warning::new(
                lines[2].clone(),
                "DuplicatedKey",
                "The FOO key is duplicated",
            ),
            Warning::new(lines[3].clone(), "DuplicatedKey", "The Z key is duplicated"),
        ];

        assert_eq!(
            Some(2),
            fixer.fix_warnings(warnings.iter_mut().collect(), &mut lines)
        );
        // what needed to be changed is changed
        assert_eq!(lines[2], line_entry(3, 4, "# FOO=BAZ"));
        assert_eq!(lines[3], line_entry(4, 4, "# Z=X"));
        // anything else left untouched
        assert_eq!(
            &lines[..2],
            &[line_entry(1, 4, "FOO=BAR"), line_entry(2, 4, "Z=Y")]
        );
    }

    #[test]
    fn fix_lines_without_warnings() {
        let mut fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 4, "FOO=BAR"),
            line_entry(2, 4, "FOO=BAZ"),
            line_entry(3, 4, "Z=Y"),
            line_entry(4, 4, "Z=X"),
        ];
        let mut warnings = vec![];

        assert_eq!(
            Some(0),
            fixer.fix_warnings(warnings.iter_mut().collect(), &mut lines)
        );
        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert_eq!("# FOO=BAZ", lines[1].raw_string);
        assert_eq!("Z=Y", lines[2].raw_string);
        assert_eq!("# Z=X", lines[3].raw_string);
    }

    #[test]
    fn control_comment_at_first_line() {
        let mut fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 5, "# dotenv-linter:off DuplicatedKey"),
            line_entry(2, 5, "FOO=BAR"),
            line_entry(3, 5, "FOO=BAZ"),
            line_entry(4, 5, "Z=Y"),
            line_entry(5, 5, "Z=X"),
        ];
        let mut warnings = vec![];

        assert_eq!(
            Some(0),
            fixer.fix_warnings(warnings.iter_mut().collect(), &mut lines)
        );
        assert_eq!("# dotenv-linter:off DuplicatedKey", lines[0].raw_string);
        assert_eq!("FOO=BAR", lines[1].raw_string);
        assert_eq!("FOO=BAZ", lines[2].raw_string);
        assert_eq!("Z=Y", lines[3].raw_string);
        assert_eq!("Z=X", lines[4].raw_string);
    }

    #[test]
    fn control_comment_in_the_middle() {
        let mut fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 6, "FOO=BAR"),
            line_entry(2, 6, "# dotenv-linter:off DuplicatedKey"),
            line_entry(3, 6, "FOO=BAZ"),
            line_entry(4, 6, "Z=Y"),
            line_entry(5, 6, "# dotenv-linter:on DuplicatedKey"),
            line_entry(6, 6, "Z=X"),
        ];
        let mut warnings = vec![];

        assert_eq!(
            Some(0),
            fixer.fix_warnings(warnings.iter_mut().collect(), &mut lines)
        );
        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert_eq!("# dotenv-linter:off DuplicatedKey", lines[1].raw_string);
        assert_eq!("FOO=BAZ", lines[2].raw_string);
        assert_eq!("Z=Y", lines[3].raw_string);
        assert_eq!("# dotenv-linter:on DuplicatedKey", lines[4].raw_string);
        assert_eq!("Z=X", lines[5].raw_string);
    }

    #[test]
    fn unrelated_control_comment() {
        let mut fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 5, "# dotenv-linter:off LowercaseKey"),
            line_entry(2, 5, "FOO=BAR"),
            line_entry(3, 5, "FOO=BAZ"),
            line_entry(4, 5, "Z=Y"),
            line_entry(5, 5, "Z=X"),
        ];
        let mut warnings = vec![];

        assert_eq!(
            Some(0),
            fixer.fix_warnings(warnings.iter_mut().collect(), &mut lines)
        );
        assert_eq!("# dotenv-linter:off LowercaseKey", lines[0].raw_string);
        assert_eq!("FOO=BAR", lines[1].raw_string);
        assert_eq!("# FOO=BAZ", lines[2].raw_string);
        assert_eq!("Z=Y", lines[3].raw_string);
        assert_eq!("# Z=X", lines[4].raw_string);
    }
}
