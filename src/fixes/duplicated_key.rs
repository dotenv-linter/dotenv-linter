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

    fn is_mandatory(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;
    use crate::fixes::run_fix_warnings;
    use crate::lines_and_warnings;

    #[test]
    fn fix_warnings() {
        let mut fixer = DuplicatedKeyFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "FOO=BAR" => None,
            "Z=Y" => None,
            "FOO=BAZ" => Some(("Duplicatedkey", "The Foo key is duplicated")),
            "Z=X" => Some(("Duplicatedkey", "The Z key is duplicated")),
        ];
        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(2), fix_count);
        assert_eq!(vec!["FOO=BAR", "Z=Y", "# FOO=BAZ", "# Z=X"], fixed_lines);
    }

    #[test]
    fn fix_lines_without_warnings() {
        let mut fixer = DuplicatedKeyFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "FOO=BAR" => None,
            "FOO=BAZ" => None,
            "Z=Y" => None,
            "Z=X" => None,
        ];
        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(0), fix_count);
        assert_eq!(vec!["FOO=BAR", "# FOO=BAZ", "Z=Y", "# Z=X"], fixed_lines);
    }

    #[test]
    fn control_comment_at_first_line() {
        let mut fixer = DuplicatedKeyFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "# dotenv-linter:off DuplicatedKey" => None,
            "FOO=BAR" => None,
            "FOO=BAZ" => None,
            "Z=Y" => None,
            "Z=X" => None,
        ];
        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(0), fix_count);
        assert_eq!(
            vec![
                "# dotenv-linter:off DuplicatedKey",
                "FOO=BAR",
                "FOO=BAZ",
                "Z=Y",
                "Z=X"
            ],
            fixed_lines
        );
    }

    #[test]
    fn control_comment_in_the_middle() {
        let mut fixer = DuplicatedKeyFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "FOO=BAR" => None,
            "# dotenv-linter:off DuplicatedKey" => None,
            "FOO=BAZ" => None,
            "Z=Y" => None,
            "# dotenv-linter:on DuplicatedKey" => None,
            "Z=X" => None,
        ];

        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(0), fix_count);
        assert_eq!(
            vec![
                "FOO=BAR",
                "# dotenv-linter:off DuplicatedKey",
                "FOO=BAZ",
                "Z=Y",
                "# dotenv-linter:on DuplicatedKey",
                "Z=X"
            ],
            fixed_lines
        );
    }

    #[test]
    fn unrelated_control_comment() {
        let mut fixer = DuplicatedKeyFixer::default();

        let (lines, warnings) = lines_and_warnings![
            "# dotenv-linter:off LowercaseKey" => None,
            "FOO=BAR" => None,
            "FOO=BAZ" => None,
            "Z=Y" => None,
            "Z=X" => None,
        ];
        let (fix_count, fixed_lines) = run_fix_warnings(&mut fixer, lines, warnings);

        assert_eq!(Some(0), fix_count);
        assert_eq!(
            vec![
                "# dotenv-linter:off LowercaseKey",
                "FOO=BAR",
                "# FOO=BAZ",
                "Z=Y",
                "# Z=X"
            ],
            fixed_lines
        );
    }
}
