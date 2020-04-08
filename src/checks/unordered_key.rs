use crate::checks::Check;
use crate::common::*;

pub(crate) struct UnorderedKeyChecker {
    template: String,
    keys: Vec<String>,
    name: String,
}

impl UnorderedKeyChecker {
    fn message(&self, key_one: &str, key_two: &str) -> String {
        return format!(
            "{}: {}",
            self.name,
            self.template
                .replace("{1}", key_one)
                .replace("{2}", key_two)
        );
    }
}

impl Default for UnorderedKeyChecker {
    fn default() -> Self {
        Self {
            keys: Vec::new(),
            name: String::from("UnorderedKey"),
            template: String::from("The {1} key should go before the {2} key"),
        }
    }
}

impl Check for UnorderedKeyChecker {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let key = line.get_key()?;
        self.keys.push(key.clone());
        let mut sorted_keys = self.keys.clone();
        sorted_keys.sort();

        if !sorted_keys.eq(&self.keys) {
            let index = sorted_keys.iter().position(|p| p == &key)?;

            let another_key = sorted_keys.get(index + 1)?;

            let warning = Warning::new(line.clone(), self.message(&key, &another_key));
            return Some(warning);
        }

        None
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
                file_path: PathBuf::from(".env"),
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
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_path: PathBuf::from(".env"),
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
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("BAR=FOO"),
                    },
                    String::from("UnorderedKey: The BAR key should go before the FOO key"),
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
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("BAR=FOO"),
                    },
                    String::from("UnorderedKey: The BAR key should go before the FOO key"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("ABC=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 3,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("ABC=BAR"),
                    },
                    String::from("UnorderedKey: The ABC key should go before the BAR key"),
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
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("BAR=FOO"),
                    },
                    String::from("UnorderedKey: The BAR key should go before the FOO key"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("DDD=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 3,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("DDD=BAR"),
                    },
                    String::from("UnorderedKey: The DDD key should go before the FOO key"),
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
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("BAR=FOO"),
                    },
                    String::from("UnorderedKey: The BAR key should go before the FOO key"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("DDD=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 3,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("DDD=BAR"),
                    },
                    String::from("UnorderedKey: The DDD key should go before the FOO key"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_path: PathBuf::from(".env"),
                    raw_string: String::from("ZOO=BAR"),
                },
                None,
            ),
        ];

        run_unordered_tests(asserts);
    }
}
