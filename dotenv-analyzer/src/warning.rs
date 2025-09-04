use crate::lint_kind::LintKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Warning {
    check_name: LintKind,
    line_number: usize,
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

    pub fn check_name(&self) -> &LintKind {
        &self.check_name
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
