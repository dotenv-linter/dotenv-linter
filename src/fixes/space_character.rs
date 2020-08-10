use super::Fix;
use crate::common::*;

pub(crate) struct SpaceCharacterFixer<'a> {
    name: &'a str,
}

impl Default for SpaceCharacterFixer<'_> {
    fn default() -> Self {
        Self {
            name: "SpaceCharacter",
        }
    }
}

impl Fix for SpaceCharacterFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;
        let value = line.get_value()?;
        line.raw_string = format!("{}={}", key.trim_end(), value.trim_start());

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn fix_line_test() {
        let fixer = SpaceCharacterFixer::default();
        let mut line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("FOO = BAR"),
        };
        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=BAR", line.raw_string);
    }

    #[test]
    fn trailing_should_not_be_fixed() {
        let fixer = SpaceCharacterFixer::default();
        let mut line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("DEBUG_HTTP=true "),
        };
        assert_eq!("DEBUG_HTTP=true ", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = SpaceCharacterFixer::default();
        let mut lines = vec![
            LineEntry {
                number: 1,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 3,
                },
                raw_string: String::from("FOO= BAR"),
            },
            LineEntry {
                number: 2,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 3,
                },
                raw_string: String::from("Z =Y"),
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
        let mut warnings = vec![
            Warning::new(
                lines[0].clone(),
                "SpaceCharacter",
                String::from("The line has spaces around equal sign"),
            ),
            Warning::new(
                lines[1].clone(),
                "SpaceCharacter",
                String::from("The line has spaces around equal sign"),
            ),
        ];

        assert_eq!(
            Some(2),
            fixer.fix_warnings(warnings.iter_mut().collect(), &mut lines)
        );
        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert_eq!("Z=Y", lines[1].raw_string);
    }
}
