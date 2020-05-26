use super::Fix;
use crate::common::*;

pub(crate) struct ExtraBlankLineFixer<'a> {
    name: &'a str,
}

impl Default for ExtraBlankLineFixer<'_> {
    fn default() -> Self {
        Self {
            name: "ExtraBlankLine",
        }
    }
}

impl Fix for ExtraBlankLineFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_warnings(
        &self,
        warnings: Vec<&mut Warning>,
        lines: &mut Vec<LineEntry>,
    ) -> Option<usize> {
        lines.dedup_by(|a, b| a.is_empty() && b.is_empty());

        self.set_fixed_on_all(warnings)
    }
}
