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
    fn run(&mut self, line: LineEntry) -> Option<Warning> {
        let key = line.get_key()?;

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
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_name: String::from(".env"),
                        raw_string: String::from("FOO=BAR"),
                    },
                    String::from("The FOO key is duplicated"),
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
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_name: String::from(".env"),
                        raw_string: String::from("FOO=BAR"),
                    },
                    String::from("The FOO key is duplicated"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_name: String::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 4,
                    file_name: String::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 4,
                        file_name: String::from(".env"),
                        raw_string: String::from("BAR=FOO"),
                    },
                    String::from("The BAR key is duplicated"),
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
                    raw_string: String::from("FOO=BAR"),
                },
                None,
            ),
            (
                LineEntry {
                    number: 2,
                    file_name: String::from(".env"),
                    raw_string: String::from("FOO=BAR"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_name: String::from(".env"),
                        raw_string: String::from("FOO=BAR"),
                    },
                    String::from("The FOO key is duplicated"),
                )),
            ),
            (
                LineEntry {
                    number: 3,
                    file_name: String::from(".env"),
                    raw_string: String::from("BAR=FOO"),
                },
                None,
            ),
        ];

        run_tests(tests);
    }
}
