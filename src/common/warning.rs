use std::fmt;

use crate::common::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Warning {
    pub check_name: String,
    line: LineEntry,
    message: String,
}

impl Warning {
    pub fn new(line: LineEntry, check_name: &str, message: String) -> Self {
        let check_name = String::from(check_name);
        Self {
            line,
            check_name,
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
            "{}:{} {}: {}",
            self.line.file,
            self.line.number.to_string().italic(),
            self.check_name.red().bold(),
            self.message
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
        let warning = Warning::new(
            line,
            "DuplicatedKey",
            String::from("The FOO key is duplicated"),
        );

        assert_eq!(
            format!(
                "{} {} {}",
                format!("{}:{}", ".env", "1").italic(),
                "DuplicatedKey:".red().bold(),
                "The FOO key is duplicated"
            ),
            format!("{}", warning)
        );
    }
}
