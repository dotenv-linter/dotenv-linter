use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    line: LineEntry,
    message: String,
}

impl Warning {
    pub fn new(line: LineEntry, message: String) -> Self {
        Self { line, message }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} {}",
            self.line.file_name, self.line.number, self.message
        )
    }
}

pub struct FileEntry {
    pub lines: Vec<LineEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineEntry {
    pub number: usize,
    pub file_name: String,
    pub raw_string: String,
}

impl LineEntry {
    pub fn get_key(&self) -> Option<String> {
        if self.raw_string.is_empty() {
            return None;
        }

        match self.raw_string.find('=') {
            Some(index) => Some(self.raw_string[..index].to_owned()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_key {
        use super::*;

        #[test]
        fn empty_line_test() {
            let input = LineEntry {
                number: 1,
                file_name: String::from(".env"),
                raw_string: String::from(""),
            };
            let expected = None;

            assert_eq!(expected, input.get_key());
        }

        #[test]
        fn correct_line_test() {
            let input = LineEntry {
                number: 1,
                file_name: String::from(".env"),
                raw_string: String::from("FOO=BAR"),
            };
            let expected = Some(String::from("FOO"));

            assert_eq!(expected, input.get_key());
        }

        #[test]
        fn line_without_value_test() {
            let input = LineEntry {
                number: 1,
                file_name: String::from(".env"),
                raw_string: String::from("FOO="),
            };
            let expected = Some(String::from("FOO"));

            assert_eq!(expected, input.get_key());
        }

        #[test]
        fn missing_value_and_equal_sign_test() {
            let input = LineEntry {
                number: 1,
                file_name: String::from(".env"),
                raw_string: String::from("FOOBAR"),
            };
            let expected = None;

            assert_eq!(expected, input.get_key());
        }
    }
}
