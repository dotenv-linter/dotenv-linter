use std::fmt;

use crate::common::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    pub check_name: String,
    line: LineEntry,
    message: String,
    pub is_fixed: bool,
}

impl Warning {
    pub fn new(line: LineEntry, check_name: &str, message: String) -> Self {
        let check_name = String::from(check_name);
        Self {
            line,
            check_name,
            message,
            is_fixed: false,
        }
    }

    pub fn line_number(&self) -> usize {
        self.line.number
    }

    pub fn mark_as_fixed(&mut self) {
        self.is_fixed = true;
    }

    pub fn mark_as_unfixed(&mut self) {
        self.is_fixed = false;
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} {}: {}",
            self.line.file, self.line.number, self.check_name, self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn warning_fmt_test() {
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("FOO=BAR"),
        };
        let warning = Warning::new(
            line,
            "DuplicatedKey",
            String::from("The FOO key is duplicated"),
        );

        assert_eq!(
            ".env:1 DuplicatedKey: The FOO key is duplicated",
            format!("{}", warning)
        );
    }
}
