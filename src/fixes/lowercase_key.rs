use super::Fix;
use crate::common::*;

pub(crate) struct LowercaseKeyFixer<'a> {
    name: &'a str,
}

impl Default for LowercaseKeyFixer<'_> {
    fn default() -> Self {
        Self {
            name: "LowercaseKey",
        }
    }
}

impl Fix for LowercaseKeyFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        let key = line.get_key()?;
        let key = key.to_uppercase();
        line.raw_string = format!("{}={}", key, line.get_value()?);

        Some(())
    }
}
