use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    line: LineEntry,
    message: String,
}

impl Warning {
    pub fn new(line: LineEntry, message: String) -> Self {
        Self { line, message }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} {}",
            self.line.file_path.display(),
            self.line.number,
            self.message
        )
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileEntry {
    pub path: PathBuf,
    pub file_name: String,
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
    pub file_path: PathBuf,
    pub raw_string: String,
}

impl LineEntry {
    pub fn is_empty_or_comment(&self) -> bool {
        let trimmed_string = self.raw_string.trim();

        trimmed_string.is_empty() || trimmed_string.starts_with('#')
    }

    pub fn get_key(&self) -> Option<String> {
        if self.is_empty_or_comment() {
            return None;
        }

        match self.raw_string.find('=') {
            Some(index) => Some(self.raw_string[..index].to_owned()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warning_fmt_test() {
        let line = LineEntry {
            number: 1,
            file_path: PathBuf::from(".env"),
            raw_string: String::from("FOO=BAR"),
        };
        let warning = Warning::new(
            line,
            String::from("DuplicatedKey: The FOO key is duplicated"),
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
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from(""),
                };

                assert_eq!(input.is_empty_or_comment(), true);
            }

            #[test]
            fn run_with_comment_line_test() {
                let input = LineEntry {
                    number: 1,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("# Comment"),
                };

                assert_eq!(input.is_empty_or_comment(), true);
            }

            #[test]
            fn run_with_not_comment_or_empty_line_test() {
                let input = LineEntry {
                    number: 1,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("NotComment"),
                };

                assert_eq!(input.is_empty_or_comment(), false);
            }
        }

        mod get_key {
            use super::*;

            #[test]
            fn empty_line_test() {
                let input = LineEntry {
                    number: 1,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from(""),
                };
                let expected = None;

                assert_eq!(expected, input.get_key());
            }

            #[test]
            fn correct_line_test() {
                let input = LineEntry {
                    number: 1,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                };
                let expected = Some(String::from("FOO"));

                assert_eq!(expected, input.get_key());
            }

            #[test]
            fn line_without_value_test() {
                let input = LineEntry {
                    number: 1,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("FOO="),
                };
                let expected = Some(String::from("FOO"));

                assert_eq!(expected, input.get_key());
            }

            #[test]
            fn missing_value_and_equal_sign_test() {
                let input = LineEntry {
                    number: 1,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("FOOBAR"),
                };
                let expected = None;

                assert_eq!(expected, input.get_key());
            }
        }
    }
}
