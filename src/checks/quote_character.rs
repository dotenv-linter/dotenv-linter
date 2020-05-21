use crate::checks::Check;
use crate::common::*;

pub(crate) struct QuoteCharacterChecker {
    name: &'static str,
    template: &'static str,
}

impl QuoteCharacterChecker {
    fn message(&self) -> String {
        format!("{}: {}", self.name, self.template)
    }
}

impl Default for QuoteCharacterChecker {
    fn default() -> Self {
        Self {
            name: "QuoteCharacter",
            template: "The value is wrapped in quotes",
        }
    }
}

impl Check for QuoteCharacterChecker {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let val = line.get_value()?;
        if val.contains('\"') || val.contains('\'') {
            Some(Warning::new(line.clone(), self.message()))
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn run_quote_char_tests(asserts: Vec<(LineEntry, Option<Warning>)>) {
        let mut checker = QuoteCharacterChecker::default();

        for assert in asserts {
            let (input, output) = assert;
            assert_eq!(checker.run(&input), output);
        }
    }

    #[test]
    fn with_single_quote_test() {
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
                    raw_string: String::from("FOO='BAR'"),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("FOO='BAR'"),
                    },
                    String::from("QuoteCharacter: The value is wrapped in quotes"),
                )),
            ),
        ];

        run_quote_char_tests(asserts);
    }

    #[test]
    fn with_double_quote_test() {
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
                    raw_string: String::from("FOO=\"BAR\""),
                },
                Some(Warning::new(
                    LineEntry {
                        number: 2,
                        file_path: PathBuf::from(".env"),
                        raw_string: String::from("FOO=\"BAR\""),
                    },
                    String::from("QuoteCharacter: The value is wrapped in quotes"),
                )),
            ),
        ];

        run_quote_char_tests(asserts);
    }

    #[test]
    fn with_no_quotes_test() {
        let asserts = vec![(
            LineEntry {
                number: 1,
                file_path: PathBuf::from(".env"),
                raw_string: String::from("FOO=BAR"),
            },
            None,
        )];

        run_quote_char_tests(asserts);
    }
}
