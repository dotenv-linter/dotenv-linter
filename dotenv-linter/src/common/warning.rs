use super::LintKind;
use owo_colors::{OwoColorize, Stream, Style};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
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
            self.line_number
                .to_string()
                .if_supports_color(Stream::Stdout, |text| text.italic()),
            self.check_name
                .to_string()
                .if_supports_color(Stream::Stdout, |text| text.style(Style::new().red().bold())),
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
                format!("{}", 1).if_supports_color(Stream::Stdout, |text| text.italic()),
                LintKind::DuplicatedKey
                    .to_string()
                    .if_supports_color(Stream::Stdout, |test| test
                        .style(Style::new().red().bold())),
                "The FOO key is duplicated"
            ),
            format!("{}", warning)
        );
    }
}
