use std::fmt;

use crate::{common::*, lints::*};

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    pub check_name: String,
    line: LineEntry,
    message: LintKind,
}

impl Warning {
    pub fn new(line: LineEntry, check_name: impl Into<String>, message: impl Into<String>) -> Self {
        let check_name = check_name.into();
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
            self.check_name.red().bold(),
            self.message.into()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::*;

    #[test]
    fn warning_fmt_test() {
        let line = line_entry(1, 1, "FOO=BAR");
        let warning = Warning::new(line, "DuplicatedKey", "The FOO key is duplicated");

        assert_eq!(
            format!(
                "{} {}: {}",
                format!("{}:{}", ".env", "1").italic(),
                "DuplicatedKey".red().bold(),
                "The FOO key is duplicated"
            ),
            format!("{}", warning)
        );
    }
}
