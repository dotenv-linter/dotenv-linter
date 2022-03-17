use crate::common::TestDir;

#[test]
fn short_version() {
    let testdir = TestDir::new();
    let expected = format!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    testdir.test_command_success_with_args(["-v"], expected);
}
