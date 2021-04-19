use crate::checks::Check;
use crate::common::*;

pub(crate) struct EndingBlankLineChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl Default for EndingBlankLineChecker<'_> {
    fn default() -> Self {
        Self {
            name: "EndingBlankLine",
            template: "No blank line at the end of the file",
        }
    }
}

impl EndingBlankLineChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Check for EndingBlankLineChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if line.is_last_line() && !line.raw_string.ends_with(LF) {
            Some(Warning::new(line.clone(), self.name(), self.message()))
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        self.name
    }

    fn skip_comments(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{check_tester, common::tests::*};

    check_tester! {
        EndingBlankLineChecker;
        blank_line => {
            "\n" => None,
        },
        blank_line_rn => {
            "\r\n" => None,
        },
        no_blank_line => {
            "a" => Some("No blank line at the end of the file"),
        }
    }
}
