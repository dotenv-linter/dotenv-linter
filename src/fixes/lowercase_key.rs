use super::Fix;
use crate::common::*;

pub(crate) struct LowercaseKeyFixer<'a> {
    name: &'a str,
}

impl Default for LowercaseKeyFixer<'_> {
    fn default() -> Self {
        Self {
            name: "LowercaseKey",
        }
    }
}

impl Fix for LowercaseKeyFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;
        let key = key.to_uppercase();
        line.raw_string = format!("{}={}", key, line.get_value()?);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn fix_line_test() {
        let fixer = LowercaseKeyFixer::default();
        let mut line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("foO=BAR"),
        };
        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=BAR", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = LowercaseKeyFixer::default();
        let mut lines = vec![
            LineEntry {
                number: 1,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 3,
                },
                raw_string: String::from("foO=BAR"),
            },
            LineEntry {
                number: 2,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 3,
                },
                raw_string: String::from("Z=Y"),
            },
            LineEntry {
                number: 3,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 3,
                },
                raw_string: String::from("\n"),
            },
        ];
        let mut warning = Warning::new(
            lines[0].clone(),
            "LowercaseKey",
            String::from("The FOO key should be in uppercase"),
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert!(warning.is_fixed);
    }
}
