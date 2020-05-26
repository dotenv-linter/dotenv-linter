use super::Fix;
use crate::common::*;

pub(crate) struct EndingBlankLineFixer<'a> {
    name: &'a str,
}

impl Default for EndingBlankLineFixer<'_> {
    fn default() -> Self {
        Self {
            name: "EndingBlankLine",
        }
    }
}

impl Fix for EndingBlankLineFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_warnings(
        &self,
        warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        if warnings.len() != 1 {
            return None;
        }

        for warning in warnings {
            lines.push(LineEntry {
                number: warning.line_number() + 1,
                file: warning.file().clone(),
                raw_string: String::new(),
            });
            warning.set_fixed(true);
        }

        Some(1)
    }
}
