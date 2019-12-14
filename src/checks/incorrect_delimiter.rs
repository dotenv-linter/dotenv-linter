use crate::checks::{Check, Warning};
use crate::LineEntry;

pub(crate) struct IncorrectDelimiterChecker {
    warning: Warning,
}

impl Check for IncorrectDelimiterChecker {
    fn run(&self, line: &LineEntry) -> Option<Warning> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incorrect_delimiter_checker_run() {}
}
