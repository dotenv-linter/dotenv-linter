use std::fmt;
use std::path::PathBuf;

use crate::common::*;

// A structure used to compare environment files
pub struct CompareFileType {
    pub path: PathBuf,
    pub keys: Vec<String>,
    pub missing: Vec<String>,
}

pub struct CompareWarning {
    pub path: PathBuf,
    pub missing_keys: Vec<String>,
}

impl fmt::Display for CompareWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{} is missing keys: {}",
                self.path.display(),
                self.missing_keys.join(", ")
            )
            .italic(),
        )
    }
}
