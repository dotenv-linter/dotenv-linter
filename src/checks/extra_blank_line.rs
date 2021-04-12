use crate::checks::Check;
use crate::common::*;

pub(crate) struct ExtraBlankLineChecker<'a> {
    template: &'a str,
    name: &'a str,
    last_blank_number: Option<usize>,
}

impl ExtraBlankLineChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for ExtraBlankLineChecker<'_> {
    fn default() -> Self {
        Self {
            template: "Extra blank line detected",
            name: "ExtraBlankLine",
            last_blank_number: None,
        }
    }
}

impl Check for ExtraBlankLineChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if !line.is_empty() {
            return None;
        }

        let is_extra = self
            .last_blank_number
            .map_or(false, |n| n + 1 == line.number);

        self.last_blank_number = Some(line.number);

        if is_extra {
            return Some(Warning::new(line.clone(), self.name(), self.message()));
        }

        None
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{check_tester, common::tests::*};

    check_tester!{
        ExtraBlankLineChecker;
        no_blank_lines => {
            "A=B" => None,
            "C=D" => None,
        },
        single_blank_line => {
            "A=B"   => None,
            ""      => None,
            "C=D"   => None,
        },
        two_blank_lines => {
            "A=B"   => None,
            ""      => None,
            ""      => Some("Extra blank line detected"),
            "C=D"   => None,
        },
        three_blank_lines => {
            "A=B"   => None,
            ""      => None,
            ""      => Some("Extra blank line detected"),
            ""      => Some("Extra blank line detected"),
            "C=D"   => None,
        }
    }
}
