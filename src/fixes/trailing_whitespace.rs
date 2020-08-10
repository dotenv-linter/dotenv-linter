use super::Fix;
use crate::common::*;

pub(crate) struct TrailingWhitespaceFixer<'a> {
    name: &'a str,
}

impl Default for TrailingWhitespaceFixer<'_> {
    fn default() -> Self {
        Self {
            name: "TrailingWhitespace",
        }
    }
}

impl Fix for TrailingWhitespaceFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        line.raw_string = line.raw_string.trim_end().to_string();

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn fix_line_test() {
        let fixer = TrailingWhitespaceFixer::default();

        let mut line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("DEBUG_HTTP=true  "),
        };

        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("DEBUG_HTTP=true", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = TrailingWhitespaceFixer::default();
        let mut lines = vec![
            LineEntry {
                number: 1,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 3,
                },
                raw_string: String::from("FOO=BAR "),
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
            "TrailingWhitespace",
            String::from("Trailing whitespace detected"),
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert!(warning.is_fixed);
    }
}
