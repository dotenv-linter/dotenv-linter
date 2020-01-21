use crate::checks::Check;
use crate::common::*;

pub(crate) struct UnorderedKeysChecker {
    template: String,
    keys: Vec<String>,
}

impl Default for UnorderedKeysChecker {
    fn default() -> Self {
        Self {
            keys: Vec::new(),
            template: String::from("The {1} key should go before the {2} key"),
        }
    }
}

impl Check for UnorderedKeysChecker {
    fn run(&mut self, line: LineEntry) -> Option<Warning> {
        let key = line.get_key()?;
        self.keys.push(key.clone());
        let mut sorted_keys = self.keys.clone();
        sorted_keys.sort();

        if !sorted_keys.eq(&self.keys) {
            let index = sorted_keys.iter().position(|p| p == &key)?;

            if (index + 1) >= sorted_keys.len() {
                // If the vectors are not equal, but the key is the last element
                return None;
            }

            let another_key = sorted_keys[index + 1].clone();

            let warning = Warning::new(
                line,
                self.template
                    .replace("{1}", &key)
                    .replace("{2}", &another_key),
            );
            return Some(warning);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_unordered_tests(asserts: Vec<(LineEntry, Option<Warning>)>) {
        let mut checker = UnorderedKeysChecker::default();

        for assert in asserts {
            let (input, output) = assert;
            assert_eq!(checker.run(input), output);
        }
    }

    #[test]
    fn one_key_test() {
        let asserts = vec![(
            LineEntry {
                number: 1,
                file_name: String::from(".env"),
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
                    file_name: String::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
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
                    file_name: String::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_name: String::from(".env"),
                        raw_string: String::from("BAR=FOO"),
                    },
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
                    file_name: String::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_name: String::from(".env"),
                        raw_string: String::from("BAR=FOO"),
                    },
                    String::from("The BAR key should go before the FOO key"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_name: String::from(".env"),
                    raw_string: String::from("ABC=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 3,
                        file_name: String::from(".env"),
                        raw_string: String::from("ABC=BAR"),
                    },
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
                    file_name: String::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_name: String::from(".env"),
                        raw_string: String::from("BAR=FOO"),
                    },
                    String::from("The BAR key should go before the FOO key"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_name: String::from(".env"),
                    raw_string: String::from("DDD=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 3,
                        file_name: String::from(".env"),
                        raw_string: String::from("DDD=BAR"),
                    },
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
                    file_name: String::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_name: String::from(".env"),
                        raw_string: String::from("BAR=FOO"),
                    },
                    String::from("The BAR key should go before the FOO key"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_name: String::from(".env"),
                    raw_string: String::from("DDD=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 3,
                        file_name: String::from(".env"),
                        raw_string: String::from("DDD=BAR"),
                    },
                    String::from("The DDD key should go before the FOO key"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_name: String::from(".env"),
                    raw_string: String::from("ZOO=BAR"),
                },
                None,
            ),
        ];

        run_unordered_tests(asserts);
    }
}
