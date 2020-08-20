use crate::checks::Check;
use crate::common::*;
use std::collections::HashSet;

pub(crate) struct DuplicatedKeyChecker<'a> {
    name: &'a str,
    template: &'a str,
    keys: HashSet<String>,
}

impl DuplicatedKeyChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", &key)
    }
}

impl Default for DuplicatedKeyChecker<'_> {
    fn default() -> Self {
        Self {
            keys: HashSet::new(),
            name: "DuplicatedKey",
            template: "The {} key is duplicated",
        }
    }
}

impl Check for DuplicatedKeyChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let key = line.get_key()?;

        if self.keys.contains(&key) {
            return Some(Warning::new(line.clone(), self.name(), self.message(&key)));
        }

        self.keys.insert(key);
        None
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn run_duplicated_tests(asserts: Vec<(LineEntry, Option<Warning>)>) {
        let mut checker = DuplicatedKeyChecker::default();

        for assert in asserts {
            let (input, output) = assert;
            assert_eq!(checker.run(&input), output);
        }
    }

    #[test]
    fn with_one_duplicated_key_test() {
        let asserts = vec![
            (
                LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 2,
                    },
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 2,
                    },
                    raw_string: String::from("FOO=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 2,
                        },
                        raw_string: String::from("FOO=BAR"),
                    },
                    "DuplicatedKey",
                    String::from("The FOO key is duplicated"),
                )),
            ),
        ];

        run_duplicated_tests(asserts);
    }

    #[test]
    fn with_two_unique_keys_test() {
        let asserts = vec![
            (
                LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 2,
                    },
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 2,
                    },
                    raw_string: String::from("BAR=FOO"),
                },
                None,
            ),
        ];

        run_duplicated_tests(asserts);
    }
    #[test]
    fn with_two_unique_keys_case_sensitive_test() {
        let asserts = vec![
            (
                LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 2,
                    },
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 2,
                    },
                    raw_string: String::from("Foo=FOO"),
                },
                None,
            ),
        ];

        run_duplicated_tests(asserts);
    }

    #[test]
    fn with_two_duplicated_keys_test() {
        let asserts = vec![
            (
                LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 4,
                    },
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 4,
                    },
                    raw_string: String::from("FOO=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 4,
                        },
                        raw_string: String::from("FOO=BAR"),
                    },
                    "DuplicatedKey",
                    String::from("The FOO key is duplicated"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 4,
                    },
                    raw_string: String::from("BAR=FOO"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 4,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 4,
                    },
                    raw_string: String::from("BAR=FOO"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 4,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 4,
                        },
                        raw_string: String::from("BAR=FOO"),
                    },
                    "DuplicatedKey",
                    String::from("The BAR key is duplicated"),
                )),
            ),
        ];

        run_duplicated_tests(asserts);
    }

    #[test]
    fn one_duplicated_and_one_unique_key_test() {
        let asserts = vec![
            (
                LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 3,
                    },
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 3,
                    },
                    raw_string: String::from("FOO=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 3,
                        },
                        raw_string: String::from("FOO=BAR"),
                    },
                    "DuplicatedKey",
                    String::from("The FOO key is duplicated"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 3,
                    },
                    raw_string: String::from("BAR=FOO"),
                },
                None,
            ),
        ];

        run_duplicated_tests(asserts);
    }
}
