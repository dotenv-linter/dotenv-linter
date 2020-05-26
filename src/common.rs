use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    pub check_name: String,
    line: LineEntry,
    message: String,
    pub is_fixed: Option<bool>,
}

impl Warning {
    pub fn new(line: LineEntry, check_name: &str, message: String) -> Self {
        let check_name = String::from(check_name);
        Self {
            line,
            check_name,
            message,
            is_fixed: None,
        }
    }

    pub fn line_number(&self) -> usize {
        self.line.number
    }

    pub fn file(&self) -> &FileEntry {
        &self.line.file
    }

    pub fn set_fixed(&mut self, val: bool) {
        self.is_fixed = Some(val);
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fixed_prefix = match self.is_fixed {
            Some(is_fixed) => {
                if is_fixed {
                    "  Fixed: "
                } else {
                    "Unfixed: "
                }
            }
            None => "",
        };

        write!(
            f,
            "{}{}:{} {}: {}",
            fixed_prefix,
            self.file(),
            self.line_number(),
            self.check_name,
            self.message
        )
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileEntry {
    pub path: PathBuf,
    pub file_name: String,
}

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl FileEntry {
    /// Converts `PathBuf` to `FileEntry`
    pub fn from(path: PathBuf) -> Option<Self> {
        let file_name = match path.file_name() {
            Some(s) => s,
            None => return None,
        };

        let file_name = match file_name.to_str() {
            Some(s) => s.to_string(),
            None => return None,
        };

        Some(FileEntry { path, file_name })
    }

    /// Checks a file name with the `.env` pattern
    pub fn is_env_file(&self) -> bool {
        let pattern = ".env";
        self.file_name.starts_with(pattern) || self.file_name.ends_with(pattern)
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
            },
            raw_string: String::from("FOO=BAR"),
        };
        let warning = Warning::new(
            line,
            "DuplicatedKey",
            String::from("The FOO key is duplicated"),
        );

        assert_eq!(
            ".env:1 DuplicatedKey: The FOO key is duplicated",
            format!("{}", warning)
        );
    }

    mod file_entry {
        use super::*;

        mod from {
            use super::*;

            #[test]
            fn path_without_file_test() {
                let f = FileEntry::from(PathBuf::from("/"));
                assert_eq!(None, f);
            }

            #[test]
            fn path_with_file_test() {
                let path = PathBuf::from(".env");
                let file_name = String::from(".env");
                let f = FileEntry::from(path.clone());
                assert_eq!(Some(FileEntry { path, file_name }), f);
            }
        }

        #[test]
        fn is_env_file_test() {
            let assertions = vec![
                (".env", true),
                ("foo.env", true),
                (".env.foo", true),
                ("env", false),
                ("env.foo", false),
                ("foo_env", false),
                ("foo-env", false),
                (".my-env-file", false),
            ];

            for (file_name, expected) in assertions {
                let f = FileEntry {
                    path: PathBuf::new(),
                    file_name: String::from(file_name),
                };

                assert_eq!(
                    expected,
                    f.is_env_file(),
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
                    },
                    raw_string: String::from("FOO=BAR\t"),
                };

                assert_eq!("FOO=BAR", entry.trimmed_string());
            }
        }
    }
}
