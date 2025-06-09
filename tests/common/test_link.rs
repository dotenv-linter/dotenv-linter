use std::os;
use std::path::Path;
use tempfile::TempDir;

/// Create a new symlink within a TempDir
#[cfg(not(windows))]
pub fn create_test_symlink(source_test_dir: &TempDir, dest: &Path) {
    os::unix::fs::symlink(source_test_dir, dest).expect("create symlink");
}
#[cfg(windows)]
pub fn create_test_symlink(source_test_dir: &TempDir, dest: &Path) {
    os::windows::fs::symlink_dir(source_test_dir, dest).expect("create symlink");
}
