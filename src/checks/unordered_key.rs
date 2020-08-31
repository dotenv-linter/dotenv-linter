use crate::checks::Check;
use crate::common::*;

pub(crate) struct UnorderedKeyChecker<'a> {
    template: &'a str,
    keys: Vec<String>,
    name: &'a str,
}

impl UnorderedKeyChecker<'_> {
    fn message(&self, key_one: &str, key_two: &str) -> String {
        self.template
            .replace("{1}", key_one)
            .replace("{2}", key_two)
    }
}

impl Default for UnorderedKeyChecker<'_> {
    fn default() -> Self {
        Self {
            name: "UnorderedKey",
            keys: Vec::new(),
            template: "The {1} key should go before the {2} key",
        }
    }
}

impl Check for UnorderedKeyChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        // Support of grouping variables through blank lines
        if line.is_empty() {
            self.keys.clear();
            return None;
        }

        let key = line.get_key()?;
        self.keys.push(key.clone());

        let mut sorted_keys = self.keys.clone();
        sorted_keys.sort();

        if sorted_keys.eq(&self.keys) {
            return None;
        }

        let another_key = sorted_keys.iter().skip_while(|&s| s != &key).nth(1)?;

        Some(Warning::new(
            line.clone(),
            self.name(),
            self.message(&key, &another_key),
        ))
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn run_unordered_tests(asserts: Vec<(LineEntry, Option<Warning>)>) {
        let mut checker = UnorderedKeyChecker::default();

        for assert in asserts {
            let (input, output) = assert;
            assert_eq!(checker.run(&input), output);
        }
    }

    #[test]
    fn one_key_test() {
        let asserts = vec![(
            LineEntry {
                number: 1,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 1,
                },
                raw_string: String::from("FOO=BAR"),
            },
            None,
        )];

        run_unordered_tests(asserts);
    }

    #[test]
    fn two_ordered_keys_test() {
        let asserts = vec![
            (
                LineEntry {
                    number: 1,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 2,
                    },
                    raw_string: String::from("BAR=FOO"),
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
                None,
            ),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn one_unordered_key_test() {
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
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 2,
                        },
                        raw_string: String::from("BAR=FOO"),
                    },
                    "UnorderedKey",
                    String::from("The BAR key should go before the FOO key"),
                )),
            ),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn two_unordered_keys_before_test() {
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
                    raw_string: String::from("BAR=FOO"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 3,
                        },
                        raw_string: String::from("BAR=FOO"),
                    },
                    "UnorderedKey",
                    String::from("The BAR key should go before the FOO key"),
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
                    raw_string: String::from("ABC=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 3,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 3,
                        },
                        raw_string: String::from("ABC=BAR"),
                    },
                    "UnorderedKey",
                    String::from("The ABC key should go before the BAR key"),
                )),
            ),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn two_unordered_keys_before_and_after_test() {
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
                    raw_string: String::from("BAR=FOO"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 3,
                        },
                        raw_string: String::from("BAR=FOO"),
                    },
                    "UnorderedKey",
                    String::from("The BAR key should go before the FOO key"),
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
                    raw_string: String::from("DDD=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 3,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 3,
                        },
                        raw_string: String::from("DDD=BAR"),
                    },
                    "UnorderedKey",
                    String::from("The DDD key should go before the FOO key"),
                )),
            ),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn two_ordered_and_two_unordered_keys_test() {
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
                    raw_string: String::from("BAR=FOO"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 4,
                        },
                        raw_string: String::from("BAR=FOO"),
                    },
                    "UnorderedKey",
                    String::from("The BAR key should go before the FOO key"),
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
                    raw_string: String::from("DDD=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 3,
                        file: FileEntry {
                            path: PathBuf::from(".env"),
                            file_name: ".env".to_string(),
                            total_lines: 4,
                        },
                        raw_string: String::from("DDD=BAR"),
                    },
                    "UnorderedKey",
                    String::from("The DDD key should go before the FOO key"),
                )),
            ),
            (
                LineEntry {
                    number: 4,
                    file: FileEntry {
                        path: PathBuf::from(".env"),
                        file_name: ".env".to_string(),
                        total_lines: 4,
                    },
                    raw_string: String::from("ZOO=BAR"),
                },
                None,
            ),
        ];

        run_unordered_tests(asserts);
    }

    #[test]
    fn one_unordered_key_with_blank_line_test() {
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
                    raw_string: String::from(""),
                },
                None,
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

        run_unordered_tests(asserts);
    }
}
