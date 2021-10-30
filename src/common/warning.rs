use colored::*;
use std::fmt;

use super::LintKind;

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    pub check_name: LintKind,
    pub line_number: usize,
    message: String,
}

impl Warning {
    pub fn new(line_number: usize, check_name: LintKind, message: impl Into<String>) -> Self {
        let message = message.into();
        Self {
            check_name,
            line_number,
            message,
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            format!("{}", self.line_number).italic(),
            self.check_name.to_string().red().bold(),
            self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warning_fmt_test() {
        let warning = Warning::new(1, LintKind::DuplicatedKey, "The FOO key is duplicated");

        assert_eq!(
            format!(
                "{} {}: {}",
                format!("{}", 1).italic(),
                LintKind::DuplicatedKey.to_string().red().bold(),
                "The FOO key is duplicated"
            ),
            format!("{}", warning)
        );
    }
}
