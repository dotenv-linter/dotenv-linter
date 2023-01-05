use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

/// Use as a temporary file to act on in a TestDir
pub struct TestFile {
    file_path: PathBuf,
}

impl TestFile {
    /// Create a new file and write its contents
    pub fn new(test_dir: &TempDir, name: &str, contents: &str) -> Self {
        let file_path = test_dir.path().join(name);
        let mut file = File::create(&file_path).expect("create testfile");
        file.write_all(contents.as_bytes()).expect("write to file");

        Self { file_path }
    }

    /// Get full path of TestFile on filesystem as &str
    pub fn as_str(&self) -> &str {
        self.file_path.to_str().expect("convert testfile to &str")
    }

    /// Get the filename (short path) as &str
    pub fn shortname_as_str(&self) -> &str {
        self.file_path
            .file_name()
            .expect("get shortname")
            .to_str()
            .expect("convert shortname to &str")
    }

    /// Get file contents
    pub fn contents(&self) -> String {
        fs::read_to_string(self.as_str()).expect("read file")
    }
}
