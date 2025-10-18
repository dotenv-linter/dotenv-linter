use std::path::PathBuf;

// A structure used to compare environment files
pub struct DiffFileType {
    path: PathBuf,
    keys: Vec<String>,
}

impl DiffFileType {
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

pub struct DiffWarning {
    path: PathBuf,
    missing_keys: Vec<String>,
}

impl DiffWarning {
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
