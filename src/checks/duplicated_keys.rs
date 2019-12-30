use std::collections::HashMap;

use crate::checks::{GlobalCheck, Warning};
use crate::extract_key;
use crate::LineEntry;

#[derive(Debug)]
pub(crate) struct DuplicatedKeysChecker {
    template: String,
}

impl Default for DuplicatedKeysChecker {
    fn default() -> Self {
        Self {
            template: String::from("The {} key is duplicated"),
        }
    }
}

impl GlobalCheck for DuplicatedKeysChecker {
    fn run(&self, lines: Vec<LineEntry>) -> Vec<Warning> {
        let mut keys: Vec<String> = Vec::new();
        let mut warnings: Vec<Warning> = Vec::new();

        for line in lines {
            let key = extract_key(&line.raw_string);
            if key == "" {
                continue;
            }

            keys.push(key)
        }

        let mut keyed = HashMap::new();
        for key in &keys {
            keyed.entry(key).or_insert_with(|| vec![]).push(key)
        }

        for v in keyed.values() {
            if v.len() > 1 {
                warnings.push(Warning::new(
                    LineEntry {
                        number: 1,
                        file_name: "current".to_string(),
                        raw_string: "bla".to_string(),
                    },
                    self.template.replace("{}", v[0]),
                ))
            }
        }

        warnings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicated_keys_checker_run() {
        let checker = DuplicatedKeysChecker::default();
        let lines = vec![
            LineEntry {
                number: 1,
                raw_string: String::from("RAILS_ENV=abc"),
            },
            LineEntry {
                number: 1,
                raw_string: String::from("RAILS_ENV=abc"),
            },
        ];

        let expected = vec![Warning::from("The RAILS_ENV key is duplicated")];
        assert_eq!(expected, checker.run(lines));

        let lines = vec![
            LineEntry {
                number: 1,
                raw_string: String::from("RAILS_ENV=abc"),
            },
            LineEntry {
                number: 1,
                raw_string: String::from("SOME_ENV=abc"),
            },
        ];

        let expected: Vec<Warning> = vec![];
        assert_eq!(expected, checker.run(lines));

        let lines = vec![
            LineEntry {
                number: 1,
                raw_string: String::from("RAILS_ENV=abc"),
            },
            LineEntry {
                number: 1,
                raw_string: String::from("RAILS_ENV=abc"),
            },
            LineEntry {
                number: 1,
                raw_string: String::from("NODE_ENV=abc"),
            },
            LineEntry {
                number: 1,
                raw_string: String::from("NODE_ENV=abc"),
            },
        ];

        // the order isn't predictable, so we only test for the length here
        let expected = 2;
        assert_eq!(expected, checker.run(lines).len());

        let lines = vec![
            LineEntry {
                number: 1,
                raw_string: String::from("RAILS_ENV=abc"),
            },
            LineEntry {
                number: 1,
                raw_string: String::from("RAILS_ENV=abc"),
            },
            LineEntry {
                number: 1,
                raw_string: String::from("NODE_ENV=abc"),
            },
        ];

        let expected = vec![Warning::from("The RAILS_ENV key is duplicated")];
        assert_eq!(expected, checker.run(lines));
    }
}
