use crate::checks::{Check, Warning};
use crate::LineEntry;

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
    fn run(&mut self, line: LineEntry) -> Option<Warning> {
        let key = line.extract_key()?;

        if self.keys.contains(&key) {
            let warning = Warning::new(line, self.template.replace("{}", &key));
            return Some(warning);
        }

        self.keys.push(key);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(tests: Vec<(LineEntry, Option<Warning>)>) {
        let mut checker = DuplicatedKeysChecker::default();

        for check in tests {
            let (input, output) = check;
            assert_eq!(checker.run(input), output);
        }
    }

    #[test]
    fn with_one_duplicated_key_test() {
        let tests = vec![
            (
                LineEntry {
                    number: 1,
                    file_name: String::from(".env"),
                    raw_string: String::from("RAILS_ENV=abc"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("RAILS_ENV=abc"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_name: String::from(".env"),
                        raw_string: String::from("RAILS_ENV=abc"),
                    },
                    String::from("The RAILS_ENV key is duplicated"),
                )),
            ),
        ];

        run_tests(tests);
    }

    #[test]
    fn with_two_unique_keys_test() {
        let tests = vec![
            (
                LineEntry {
                    number: 1,
                    file_name: String::from(".env"),
                    raw_string: String::from("RAILS_ENV=abc"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("SOME_ENV=abc"),
                },
                None,
            ),
        ];

        run_tests(tests);
    }

    #[test]
    fn with_two_duplicated_keys_test() {
        let tests = vec![
            (
                LineEntry {
                    number: 1,
                    file_name: String::from(".env"),
                    raw_string: String::from("RAILS_ENV=abc"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("RAILS_ENV=abc"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_name: String::from(".env"),
                        raw_string: String::from("RAILS_ENV=abc"),
                    },
                    String::from("The RAILS_ENV key is duplicated"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_name: String::from(".env"),
                    raw_string: String::from("NODE_ENV=abc"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 4,
                    file_name: String::from(".env"),
                    raw_string: String::from("NODE_ENV=abc"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 4,
                        file_name: String::from(".env"),
                        raw_string: String::from("NODE_ENV=abc"),
                    },
                    String::from("The NODE_ENV key is duplicated"),
                )),
            ),
        ];

        run_tests(tests);
    }

    #[test]
    fn one_duplicated_and_one_unique_key_test() {
        let tests = vec![
            (
                LineEntry {
                    number: 1,
                    file_name: String::from(".env"),
                    raw_string: String::from("RAILS_ENV=abc"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("RAILS_ENV=abc"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_name: String::from(".env"),
                        raw_string: String::from("RAILS_ENV=abc"),
                    },
                    String::from("The RAILS_ENV key is duplicated"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_name: String::from(".env"),
                    raw_string: String::from("NODE_ENV=abc"),
                },
                None,
            ),
        ];

        run_tests(tests);
    }
}
