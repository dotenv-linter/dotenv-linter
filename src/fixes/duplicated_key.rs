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
                if keys.contains(&key) {
                    self.fix_line(line)?;
                } else {
                    keys.insert(key);
                }
            }
        }

        let count = warnings.len();
        for warning in warnings {
            warning.mark_as_fixed();
        }

        Some(count)
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
                "The FOO key is duplicated".to_owned(),
            ),
            Warning::new(
                lines[3].clone(),
                "DuplicatedKey",
                "The Z key is duplicated".to_owned(),
            ),
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

        assert!(warnings.iter().all(|w| w.is_fixed));
    }
}
