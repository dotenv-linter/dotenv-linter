use super::Fix;
use crate::common::*;

pub(crate) struct LeadingCharacterFixer<'a> {
    name: &'a str,
}

impl Default for LeadingCharacterFixer<'_> {
    fn default() -> Self {
        Self {
            name: "LeadingCharacter",
        }
    }
}

impl Fix for LeadingCharacterFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;

        let cleaned_key = remove_invalid_leading_chars(&key);

        line.raw_string = format!("{}={}", cleaned_key, line.get_value()?);

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn fix_leading_dot() {
        let fixer = LeadingCharacterFixer::default();

        let mut leading_period = LineEntry {
            number: 2,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 7,
            },
            raw_string: String::from(".FOO=BAR"),
        };

        assert_eq!(Some(()), fixer.fix_line(&mut leading_period));
        assert_eq!("FOO=BAR", leading_period.raw_string);
    }

    #[test]
    fn fix_leading_space() {
        let fixer = LeadingCharacterFixer::default();

        let mut leading_space = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 7,
            },
            raw_string: String::from(" FOO=BAR"),
        };

        assert_eq!(Some(()), fixer.fix_line(&mut leading_space));
        assert_eq!("FOO=BAR", leading_space.raw_string);
    }

    #[test]
    fn fix_leading_asterisk() {
        let fixer = LeadingCharacterFixer::default();

        let mut leading_asterisk = LineEntry {
            number: 3,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 7,
            },
            raw_string: String::from("*FOO=BAR"),
        };

        assert_eq!(Some(()), fixer.fix_line(&mut leading_asterisk));
        assert_eq!("FOO=BAR", leading_asterisk.raw_string);
    }

    #[test]
    fn fix_leading_number() {
        let fixer = LeadingCharacterFixer::default();

        let mut leading_number = LineEntry {
            number: 4,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 7,
            },
            raw_string: String::from("1FOO=BAR"),
        };

        assert_eq!(Some(()), fixer.fix_line(&mut leading_number));
        assert_eq!("FOO=BAR", leading_number.raw_string);
    }

    #[test]
    fn fix_many_invalid_leading_chars() {
        let fixer = LeadingCharacterFixer::default();

        let mut leading_number = LineEntry {
            number: 4,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 7,
            },
            raw_string: String::from("-1&*FOO=BAR"),
        };

        assert_eq!(Some(()), fixer.fix_line(&mut leading_number));
        assert_eq!("FOO=BAR", leading_number.raw_string);
    }

    #[test]
    fn leading_underscore_is_unchanged() {
        let fixer = LeadingCharacterFixer::default();
        let mut leading_underscore = LineEntry {
            number: 5,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 7,
            },
            raw_string: String::from("_FOO=BAR"),
        };

        assert_eq!(Some(()), fixer.fix_line(&mut leading_underscore));
        assert_eq!("_FOO=BAR", leading_underscore.raw_string);
    }

    #[test]
    fn no_leading_char_is_unchanged() {
        let fixer = LeadingCharacterFixer::default();

        let mut normal = LineEntry {
            number: 6,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 7,
            },
            raw_string: String::from("FOO=BAR"),
        };

        assert_eq!(Some(()), fixer.fix_line(&mut normal));
        assert_eq!("FOO=BAR", normal.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = LeadingCharacterFixer::default();
        let mut lines = vec![
            LineEntry {
                number: 1,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 7,
                },
                raw_string: String::from(".FOO=BAR"),
            },
            LineEntry {
                number: 2,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 7,
                },
                raw_string: String::from(" Z=Y"),
            },
            LineEntry {
                number: 3,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 7,
                },
                raw_string: String::from("*BAR=BAZ"),
            },
            LineEntry {
                number: 4,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 7,
                },
                raw_string: String::from("1QUX=QUUX"),
            },
            LineEntry {
                number: 5,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 7,
                },
                raw_string: String::from("_QUUX=FOOBAR"),
            },
            LineEntry {
                number: 6,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 7,
                },
                raw_string: String::from("KEY=VALUE"),
            },
            LineEntry {
                number: 7,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 7,
                },
                raw_string: String::from("\n"),
            },
        ];

        let mut warnings = vec![
            Warning::new(
                lines[0].clone(),
                "LeadingCharacter",
                String::from("Invalid leading character detected"),
            ),
            Warning::new(
                lines[1].clone(),
                "LeadingCharacter",
                String::from("Invalid leading character detected"),
            ),
            Warning::new(
                lines[2].clone(),
                "LeadingCharacter",
                String::from("Invalid leading character detected"),
            ),
            Warning::new(
                lines[3].clone(),
                "LeadingCharacter",
                String::from("Invalid leading character detected"),
            ),
        ];

        assert_eq!(
            Some(4),
            fixer.fix_warnings(warnings.iter_mut().collect(), &mut lines)
        );

        assert_eq!("FOO=BAR", lines[0].raw_string);
        assert_eq!("Z=Y", lines[1].raw_string);
        assert_eq!("BAR=BAZ", lines[2].raw_string);
        assert_eq!("QUX=QUUX", lines[3].raw_string);
        assert_eq!("_QUUX=FOOBAR", lines[4].raw_string);
        assert_eq!("KEY=VALUE", lines[5].raw_string);
        assert_eq!("\n", lines[6].raw_string);
    }
}
