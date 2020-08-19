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
        &self,
        warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        let file = lines.first()?.file.clone();
        let last_line = lines.last()?;

        if !last_line.raw_string.ends_with(LF) {
            lines.push(LineEntry {
                number: lines.len() + 1,
                file,
                raw_string: LF.to_string(),
            });
        }

        if !warnings.is_empty() {
            for warning in warnings {
                warning.mark_as_fixed()
            }
            return Some(1);
        }

        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn fix_warnings_test() {
        let fixer = EndingBlankLineFixer::default();
        let mut lines = vec![
            LineEntry {
                number: 1,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 2,
                },
                raw_string: String::from("FOO=BAR"),
            },
            LineEntry {
                number: 2,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 2,
                },
                raw_string: String::from("Z=Y"),
            },
        ];
        let mut warning = Warning::new(
            lines[1].clone(),
            "EndingBlankLine",
            String::from("No blank line at the end of the file"),
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("\n", lines[2].raw_string);
        assert!(warning.is_fixed);
    }

    #[test]
    fn ending_blank_line_exist_test() {
        let fixer = EndingBlankLineFixer::default();
        let mut lines = vec![
            LineEntry {
                number: 1,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 2,
                },
                raw_string: String::from("FOO=BAR"),
            },
            LineEntry {
                number: 2,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 2,
                },
                raw_string: String::from(LF),
            },
        ];

        assert_eq!(Some(0), fixer.fix_warnings(vec![], &mut lines));
        assert_eq!(lines.len(), 2);
    }
}
