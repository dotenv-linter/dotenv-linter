use super::Fix;
use crate::common::*;

pub(crate) struct QuoteCharacterFixer<'a> {
    name: &'a str,
}

impl Default for QuoteCharacterFixer<'_> {
    fn default() -> Self {
        Self {
            name: "QuoteCharacter",
        }
    }
}

impl Fix for QuoteCharacterFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let value = line.get_value()?;
        let pure_val = value.replace("'", "").replace("\"", "");

        line.raw_string = format!("{}={}", line.get_key()?, pure_val);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn fix_line_test() {
        let fixer = QuoteCharacterFixer::default();
        let mut line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("FOO=\'\"b\'\"ar\"\'"),
        };
        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=bar", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = QuoteCharacterFixer::default();
        let mut lines = vec![
            LineEntry {
                number: 1,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 3,
                },
                raw_string: String::from("FOO=\"bar\'\""),
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
            "QuoteCharacter",
            String::from("The value has quote characters (\', \")"),
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("FOO=bar", lines[0].raw_string);
        assert!(warning.is_fixed);
    }
}
