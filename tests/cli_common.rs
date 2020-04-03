use assert_cmd::Command;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{tempdir, tempdir_in, TempDir};

/// Use to test commands in temporary directories
pub struct TestDir {
    current_dir: TempDir,
}

impl TestDir {
    /// Generate a new TestDir
    pub fn new() -> Self {
        Self {
            current_dir: tempdir().expect("create testdir"),
        }
    }

    /// Create a new TestDir within an existing one
    pub fn subdir(&self) -> Self {
        Self {
            current_dir: tempdir_in(&self.current_dir).expect("create subdir"),
        }
    }

    /// Explicitly panic if unable to remove TestDir from filesystem
    pub fn close(self) {
        self.current_dir.close().expect("remove testdir");
    }

    /// Get relative path between TestDir and a subdirectory TestDir
    pub fn relative_path(&self, subdir: &Self) -> String {
        subdir
            .current_dir
            .path()
            .strip_prefix(self.current_dir.path())
            .expect("strip dir from subdir path")
            .to_string_lossy()
            .to_string()
    }

    /// Create a TestFile within the TestDir
    pub fn create_testfile(&self, name: &str, contents: &str) -> TestFile {
        TestFile::new(&self.current_dir, name, contents)
    }

    /// Get full path of TestDir as a &str
    pub fn as_str(&self) -> &str {
        self.current_dir
            .path()
            .to_str()
            .expect("convert directory to &str")
    }

    /// Run the default CLI binary in this TestDir and check it succeeds.
    ///
    /// This method removes the TestDir when command has finished.
    pub fn test_command_success(self) {
        let args: &[&str; 0] = &[];
        self.test_command_success_with_args(args);
    }

    /// Run the default CLI binary in this TestDir and check it fails.
    ///
    /// This method removes the TestDir when command has finished.
    pub fn test_command_fail(self, expected_output: String) {
        let args: &[&str; 0] = &[];
        self.test_command_fail_with_args(args, expected_output);
    }

    /// Run the default CLI binary, with command line arguments,
    /// in this TestDir and check it succeeds.
    ///
    /// This method removes the TestDir when command has finished.
    pub fn test_command_success_with_args<I, S>(self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmd = Self::init_cmd();
        cmd.current_dir(&self.current_dir)
            .args(args)
            .assert()
            .success();

        self.close();
    }

    /// Run the default CLI binary, with command line arguments,
    /// in this TestDir and check it fails.
    ///
    /// This method removes the TestDir when command has finished.
    pub fn test_command_fail_with_args<I, S>(self, args: I, expected_output: String)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmd = Self::init_cmd();
        cmd.current_dir(&self.current_dir)
            .args(args)
            .assert()
            .failure()
            .code(1)
            .stdout(expected_output);

        self.close();
    }

    fn init_cmd() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("command from binary name")
    }
}

/// Use as a temporary file to act on in a TestDir
pub struct TestFile {
    file_path: PathBuf,
}

impl TestFile {
    /// Create a new file and write its contents
    pub fn new(test_dir: &TempDir, name: &str, contents: &str) -> Self {
        let file_path = test_dir.path().join(name);
        let mut file = File::create(&file_path).expect("create testfile");
        writeln!(file, "{}", contents).expect("write to file");

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
}
