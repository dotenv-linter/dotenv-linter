use crate::checks;
use crate::common::LintKind;

pub fn list() -> Vec<LintKind> {
    checks::available_check_names()
}
