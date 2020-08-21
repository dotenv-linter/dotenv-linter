use super::Fix;
use crate::common::*;

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

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        line.raw_string = format!("#{}", line.raw_string);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fix_warnings() {
        let fixer = DuplicatedKeyFixer::default();
        let mut lines = vec![
            LineEntry {
                number: 1,
                file: FileEntry {
                    path: ".env".into(),
                    file_name: ".env".to_string(),
                    total_lines: 4,
                },
                raw_string: "FOO=BAR".to_string(),
            },
            LineEntry {
                number: 2,
                file: FileEntry {
                    path: ".env".into(),
                    file_name: ".env".to_string(),
                    total_lines: 4,
                },
                raw_string: "Z=Y".to_string(),
            },
            LineEntry {
                number: 3,
                file: FileEntry {
                    path: ".env".into(),
                    file_name: ".env".to_string(),
                    total_lines: 4,
                },
                raw_string: "FOO=BAZ".to_string(),
            },
            LineEntry {
                number: 4,
                file: FileEntry {
                    path: ".env".into(),
                    file_name: ".env".to_string(),
                    total_lines: 4,
                },
                raw_string: "Z=X".to_string(),
            },
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
        assert_eq!(
            lines[2],
            LineEntry {
                number: 3,
                file: FileEntry {
                    path: ".env".into(),
                    file_name: ".env".to_string(),
                    total_lines: 4,
                },
                raw_string: "#FOO=BAZ".to_string(),
            }
        );
        assert_eq!(
            lines[3],
            LineEntry {
                number: 4,
                file: FileEntry {
                    path: ".env".into(),
                    file_name: ".env".to_string(),
                    total_lines: 4,
                },
                raw_string: "#Z=X".to_string(),
            }
        );
        // anything else left untouched
        assert_eq!(
            &lines[..2],
            &[
                LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: ".env".into(),
                        file_name: ".env".to_string(),
                        total_lines: 4,
                    },
                    raw_string: "FOO=BAR".to_string(),
                },
                LineEntry {
                    number: 2,
                    file: FileEntry {
                        path: ".env".into(),
                        file_name: ".env".to_string(),
                        total_lines: 4,
                    },
                    raw_string: "Z=Y".to_string(),
                }
            ]
        );

        assert!(warnings.iter().all(|w| w.is_fixed));
    }
}
