use crate::checks::Check;
use crate::common::*;

pub(crate) struct DuplicatedKeysChecker {
    template: String,
    keys: Vec<String>,
}

impl Default for DuplicatedKeysChecker {
    fn default() -> Self {
        Self {
            keys: Vec::new(),
            template: String::from("The {} key is duplicated"),
        }
    }
}

impl Check for DuplicatedKeysChecker {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let key = line.get_key()?;

        if self.keys.contains(&key) {
            return Some(Warning::new(
                line.clone(),
                self.template.replace("{}", &key),
            ));
        }

        self.keys.push(key);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn run_duplicated_tests(asserts: Vec<(LineEntry, Option<Warning>)>) {
        let mut checker = DuplicatedKeysChecker::default();

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
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("FOO=BAR"),
                    },
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
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("BAR=FOO"),
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
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("FOO=BAR"),
                    },
                    String::from("The FOO key is duplicated"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 4,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 4,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("BAR=FOO"),
                    },
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
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("FOO=BAR"),
                    },
                    String::from("The FOO key is duplicated"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                None,
            ),
        ];

        run_duplicated_tests(asserts);
    }
}
