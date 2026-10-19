use crate::common::TestDir;

#[test]
fn files_non_existent() {
    let test_dir = TestDir::new();
    test_dir.test_command_fail_with_args(["diff", ".env1", ".env2"], "");
}

#[test]
fn files_non_existent_in_quiet_mode() {
    let test_dir = TestDir::new();
    test_dir.test_command_fail_with_args(["diff", "--quiet", ".env1", ".env2"], "");
}
