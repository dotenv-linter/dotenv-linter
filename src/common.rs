use colored::*;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    check_name: String,
    line: LineEntry,
    message: String,
}

impl Warning {
    pub fn new(line: LineEntry, check_name: &str, message: String) -> Self {
        let check_name = String::from(check_name);
        Self {
            line,
            check_name,
            message,
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} {} {}",
            self.line.file,
            self.line.number.to_string().italic(),
            self.check_name.red().bold(),
            self.message
        )
    }
}

pub const LF: &str = "\n";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileEntry {
    pub path: PathBuf,
    pub file_name: String,
    pub total_lines: usize,
}

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display().to_string().italic())
    }
}

impl FileEntry {
    /// Converts `PathBuf` to tuple of `(FileEntry, Vec<String>)`
    pub fn from(path: PathBuf) -> Option<(Self, Vec<String>)> {
        let file_name = match Self::get_file_name(&path) {
            Some(s) => s,
            None => return None,
        };

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => return None,
        };

        let mut lines: Vec<String> = content.lines().map(|str| str.to_string()).collect();

        // You must add a line, because [`Lines`] does not return the last empty row (excludes LF)
        if content.ends_with(LF) {
            lines.push(LF.to_string());
        }

        Some((
            FileEntry {
                path,
                file_name,
                total_lines: lines.len(),
            },
            lines,
        ))
    }

    /// Checks a file name with the `.env` pattern
    pub fn is_env_file(path: &PathBuf) -> bool {
        let pattern = ".env";
        Self::get_file_name(path)
            .filter(|file_name| file_name.starts_with(pattern) || file_name.ends_with(pattern))
            .is_some()
    }

    fn get_file_name(path: &PathBuf) -> Option<String> {
        path.file_name()
            .map(|file_name| file_name.to_str())
            .unwrap_or(None)
            .map(|s| s.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineEntry {
    pub number: usize,
    pub file: FileEntry,
    pub raw_string: String,
}

impl LineEntry {
    pub fn is_empty_or_comment(&self) -> bool {
        self.is_empty() || self.is_comment()
    }

    pub fn is_empty(&self) -> bool {
        self.trimmed_string().is_empty()
    }

    pub fn is_comment(&self) -> bool {
        self.trimmed_string().starts_with('#')
    }

    pub fn get_key(&self) -> Option<String> {
        if self.is_empty_or_comment() {
            return None;
        }

        match self.trimmed_string().find('=') {
            Some(index) => Some(self.trimmed_string()[..index].to_owned()),
            None => None,
        }
    }

    pub fn get_value(&self) -> Option<String> {
        if self.is_empty_or_comment() {
            return None;
        }

        match self.raw_string.find('=') {
            Some(index) => Some(self.raw_string[(index + 1)..].to_owned()),
            None => None,
        }
    }

    pub fn trimmed_string(&self) -> &str {
        self.raw_string.trim()
    }

    pub fn is_last_line(&self) -> bool {
        self.file.total_lines == self.number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warning_fmt_test() {
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("FOO=BAR"),
        };
        let warning = Warning::new(
            line,
            "DuplicatedKey",
            String::from("The FOO key is duplicated"),
        );

        assert_eq!(
            format!(
                "{}:{} {} {}",
                ".env".italic(),
                "1".italic(),
                "DuplicatedKey".red().bold(),
                "The FOO key is duplicated"
            ),
            format!("{}", warning)
        );
    }

    mod file_entry {
        use super::*;

        mod from {
            use super::*;
            use std::env::temp_dir;
            use std::fs::remove_file;

            #[test]
            fn path_without_file_test() {
                let f = FileEntry::from(PathBuf::from("/"));
                assert_eq!(None, f);
            }

            #[test]
            fn path_with_file_test() {
                let file_name = String::from(".env");
                let path = temp_dir().join(&file_name);
                fs::File::create(&path).expect("create testfile");

                let f = FileEntry::from(path.clone());
                assert_eq!(
                    Some((
                        FileEntry {
                            path: path.clone(),
                            file_name,
                            total_lines: 0
                        },
                        vec![]
                    )),
                    f
                );
                remove_file(path).expect("temp file deleted");
            }
        }

        #[test]
        fn is_env_file_test() {
            let assertions = vec![
                (".env", true),
                ("foo.env", true),
                (".env.foo", true),
                (".env.foo.common", true),
                ("env", false),
                ("env.foo", false),
                ("foo_env", false),
                ("foo-env", false),
                (".my-env-file", false),
                ("dev.env.js", false),
            ];

            for (file_name, expected) in assertions {
                assert_eq!(
                    expected,
                    FileEntry::is_env_file(&PathBuf::from(file_name)),
                    "Expected {} for the file name {}",
                    expected,
                    file_name
                )
            }
        }
    }

    mod line_entry {
        use super::*;

        mod is_empty_or_comment {
            use super::*;

            #[test]
            fn run_with_empty_line_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from(""),
                };

                assert_eq!(input.is_empty(), true);
                assert_eq!(input.is_comment(), false);
                assert_eq!(input.is_empty_or_comment(), true);
            }

            #[test]
            fn run_with_comment_line_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("# Comment"),
                };

                assert_eq!(input.is_empty(), false);
                assert_eq!(input.is_comment(), true);
                assert_eq!(input.is_empty_or_comment(), true);
            }

            #[test]
            fn run_with_not_comment_or_empty_line_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("NotComment"),
                };

                assert_eq!(input.is_empty(), false);
                assert_eq!(input.is_comment(), false);
                assert_eq!(input.is_empty_or_comment(), false);
            }
        }

        mod get_key {
            use super::*;

            #[test]
            fn empty_line_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from(""),
                };
                let expected = None;

                assert_eq!(expected, input.get_key());
            }

            #[test]
            fn correct_line_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("FOO=BAR"),
                };
                let expected = Some(String::from("FOO"));

                assert_eq!(expected, input.get_key());
            }

            #[test]
            fn line_without_value_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("FOO="),
                };
                let expected = Some(String::from("FOO"));

                assert_eq!(expected, input.get_key());
            }

            #[test]
            fn missing_value_and_equal_sign_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("FOOBAR"),
                };
                let expected = None;

                assert_eq!(expected, input.get_key());
            }
        }

        mod get_value {
            use super::*;

            #[test]
            fn empty_line_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from(""),
                };
                let expected = None;

                assert_eq!(expected, input.get_value());
            }

            #[test]
            fn correct_line_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("FOO=BAR"),
                };
                let expected = Some(String::from("BAR"));

                assert_eq!(expected, input.get_value());
            }

            #[test]
            fn line_without_key_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("=BAR"),
                };
                let expected = Some(String::from("BAR"));

                assert_eq!(expected, input.get_value());
            }

            #[test]
            fn line_without_value_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("FOO="),
                };
                let expected = Some(String::from(""));

                assert_eq!(expected, input.get_value());
            }

            #[test]
            fn missing_value_and_equal_sign_test() {
                let input = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("FOOBAR"),
                };
                let expected = None;

                assert_eq!(expected, input.get_value());
            }
        }

        mod trimmed_string {
            use super::*;

            #[test]
            fn line_without_blank_chars_test() {
                let entry = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("FOO=BAR"),
                };

                assert_eq!("FOO=BAR", entry.trimmed_string());
            }

            #[test]
            fn line_with_spaces_test() {
                let entry = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("   FOO=BAR  "),
                };

                assert_eq!("FOO=BAR", entry.trimmed_string());
            }

            #[test]
            fn line_with_tab_test() {
                let entry = LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 1,
                    },
                    raw_string: String::from("FOO=BAR\t"),
                };

                assert_eq!("FOO=BAR", entry.trimmed_string());
            }
        }
    }
}
