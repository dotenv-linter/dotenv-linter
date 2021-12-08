use std::collections::HashSet;

use super::Fix;
use crate::common::{LineEntry, LintKind};

#[derive(Default)]
pub(crate) struct DuplicatedKeyFixer {}

impl Fix for DuplicatedKeyFixer {
    fn name(&self) -> LintKind {
        LintKind::DuplicatedKey
    }

    fn fix_warnings(&self, warning_lines: &[usize], lines: &mut Vec<LineEntry>) -> Option<usize> {
        let mut keys = HashSet::with_capacity(lines.len());
        let mut is_disabled = false;

        for line in lines {
            if let Some(comment) = line.get_control_comment() {
                if comment.checks.contains(&self.name()) {
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

        Some(warning_lines.len())
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        line.raw_string = format!("# {}", line.raw_string);

        Some(())
    }

    fn is_mandatory(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::line_entry;

    #[test]
    fn fix_warnings() {
        let fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 4, "FOO=BAR"),
            line_entry(2, 4, "Z=Y"),
            line_entry(3, 4, "FOO=BAZ"),
            line_entry(4, 4, "Z=X"),
        ];
        let warning_lines = [lines[2].number, lines[3].number];

        assert_eq!(Some(2), fixer.fix_warnings(&warning_lines, &mut lines));
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
        let fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 4, "FOO=BAR"),
            line_entry(2, 4, "FOO=BAZ"),
            line_entry(3, 4, "Z=Y"),
            line_entry(4, 4, "Z=X"),
        ];
        let warning_lines = [];

        assert_eq!(Some(0), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert_eq!("# FOO=BAZ", lines[1].raw_string);
        assert_eq!("Z=Y", lines[2].raw_string);
        assert_eq!("# Z=X", lines[3].raw_string);
    }

    #[test]
    fn control_comment_at_first_line() {
        let fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 5, "# dotenv-linter:off DuplicatedKey"),
            line_entry(2, 5, "FOO=BAR"),
            line_entry(3, 5, "FOO=BAZ"),
            line_entry(4, 5, "Z=Y"),
            line_entry(5, 5, "Z=X"),
        ];
        let warning_lines = [];

        assert_eq!(Some(0), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("# dotenv-linter:off DuplicatedKey", lines[0].raw_string);
        assert_eq!("FOO=BAR", lines[1].raw_string);
        assert_eq!("FOO=BAZ", lines[2].raw_string);
        assert_eq!("Z=Y", lines[3].raw_string);
        assert_eq!("Z=X", lines[4].raw_string);
    }

    #[test]
    fn control_comment_in_the_middle() {
        let fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 6, "FOO=BAR"),
            line_entry(2, 6, "# dotenv-linter:off DuplicatedKey"),
            line_entry(3, 6, "FOO=BAZ"),
            line_entry(4, 6, "Z=Y"),
            line_entry(5, 6, "# dotenv-linter:on DuplicatedKey"),
            line_entry(6, 6, "Z=X"),
        ];
        let warning_lines = [];

        assert_eq!(Some(0), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert_eq!("# dotenv-linter:off DuplicatedKey", lines[1].raw_string);
        assert_eq!("FOO=BAZ", lines[2].raw_string);
        assert_eq!("Z=Y", lines[3].raw_string);
        assert_eq!("# dotenv-linter:on DuplicatedKey", lines[4].raw_string);
        assert_eq!("Z=X", lines[5].raw_string);
    }

    #[test]
    fn unrelated_control_comment() {
        let fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            line_entry(1, 5, "# dotenv-linter:off LowercaseKey"),
            line_entry(2, 5, "FOO=BAR"),
            line_entry(3, 5, "FOO=BAZ"),
            line_entry(4, 5, "Z=Y"),
            line_entry(5, 5, "Z=X"),
        ];
        let warning_lines = [];

        assert_eq!(Some(0), fixer.fix_warnings(&warning_lines, &mut lines));
        assert_eq!("# dotenv-linter:off LowercaseKey", lines[0].raw_string);
        assert_eq!("FOO=BAR", lines[1].raw_string);
        assert_eq!("# FOO=BAZ", lines[2].raw_string);
        assert_eq!("Z=Y", lines[3].raw_string);
        assert_eq!("# Z=X", lines[4].raw_string);
    }
}
