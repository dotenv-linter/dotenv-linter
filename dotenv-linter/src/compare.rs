use std::path::PathBuf;

// A structure used to compare environment files
pub struct CompareFileType {
    path: PathBuf,
    keys: Vec<String>,
}

impl CompareFileType {
    pub fn new(path: PathBuf, keys: Vec<String>) -> Self {
        Self { path, keys }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn keys(&self) -> &[String] {
        &self.keys
    }
}

pub struct CompareWarning {
    path: PathBuf,
    missing_keys: Vec<String>,
}

impl CompareWarning {
    pub fn new(path: PathBuf, missing_keys: Vec<String>) -> Self {
        Self { path, missing_keys }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn missing_keys(&self) -> &[String] {
        &self.missing_keys
    }
}
