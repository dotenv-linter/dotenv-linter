use crate::checks::Check;
use crate::common::*;

pub(crate) struct SpaceCharacterChecker<'a> {
    template: &'a str,
    name: &'a str,
}

impl SpaceCharacterChecker<'_> {
    fn message(&self) -> String {
        format!("{}: {}", self.name, self.template)
    }
}

impl Default for SpaceCharacterChecker<'_> {
    fn default() -> Self {
        Self {
            name: "SpaceCharacter",
            template: "The line has spaces around equal sign",
        }
    }
}

impl Check for SpaceCharacterChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let line_splitted = line.raw_string.split('=').collect::<Vec<&str>>();

        if let [key, value] = &line_splitted[..] {
            if key.ends_with(' ') || value.starts_with(' ') {
                return Some(Warning::new(line.clone(), self.message()));
            }
        }

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

    const MESSAGE: &str = "SpaceCharacter: The line has spaces around equal sign";

    #[test]
    fn working_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_leading_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from(" DEBUG_HTTP=true"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_trailing_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("DEBUG_HTTP=true "),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_empty_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from(""),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn working_no_equal_sign_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("DEBUG_HTTP true"),
        };
        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn failing_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("DEBUG-HTTP = true"),
        };
        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn failing_when_whitespace_before_equal_sign_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("DEBUG-HTTP =true"),
        };
        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(&line));
    }

    #[test]
    fn failing_when_whitespace_after_equal_sign_run() {
        let mut checker = SpaceCharacterChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("DEBUG-HTTP= true"),
        };
        let expected = Some(Warning::new(line.clone(), MESSAGE.to_string()));
        assert_eq!(expected, checker.run(&line));
    }
}
