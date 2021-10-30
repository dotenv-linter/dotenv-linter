use colored::*;
use std::fmt;

use super::{LineEntry, LintKind};

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    pub check_name: LintKind,
    line: LineEntry,
    message: String,
}

impl Warning {
    pub fn new(line: LineEntry, check_name: LintKind, message: impl Into<String>) -> Self {
        let message = message.into();
        Self {
            check_name,
            line,
            message,
        }
    }

    pub fn line_number(&self) -> usize {
        self.line.number
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            format!("{}:{}", self.line.file, self.line.number).italic(),
            self.check_name.to_string().red().bold(),
            self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::line_entry;

    #[test]
    fn warning_fmt_test() {
        let line = line_entry(1, 1, "FOO=BAR");
        let warning = Warning::new(line, LintKind::DuplicatedKey, "The FOO key is duplicated");

        assert_eq!(
            format!(
                "{} {}: {}",
                format!("{}:{}", ".env", "1").italic(),
                LintKind::DuplicatedKey.to_string().red().bold(),
                "The FOO key is duplicated"
            ),
            format!("{}", warning)
        );
    }
}
