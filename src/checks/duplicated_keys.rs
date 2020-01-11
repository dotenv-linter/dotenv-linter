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

    #[test]
    fn duplicated_keys_checker_run() {
        let mut checker = DuplicatedKeysChecker::default();

        let checks = vec![
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

        for check in checks {
            let (input, output) = check;
            assert_eq!(checker.run(input), output);
        }

        // To clear the already saved keys
        let mut checker = DuplicatedKeysChecker::default();

        let checks = vec![
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

        for check in checks {
            let (input, output) = check;
            assert_eq!(checker.run(input), output);
        }

        // To clear the already saved keys
        let mut checker = DuplicatedKeysChecker::default();

        let checks = vec![
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

        for check in checks {
            let (input, output) = check;
            assert_eq!(checker.run(input), output);
        }

        // To clear the already saved keys
        let mut checker = DuplicatedKeysChecker::default();

        let checks = vec![
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

        for check in checks {
            let (input, output) = check;
            assert_eq!(checker.run(input), output);
        }
    }
}
