use crate::common::TestDir;

#[test]
fn files_non_existent() {
    let test_dir = TestDir::new();
    let expected_output = String::from("Nothing to compare\n");
    let output = test_dir.test_command_success_and_get_output(["compare", ".env1", ".env2"]);

    assert_eq!(output, expected_output);
}

#[test]
fn files_non_existent_in_quiet_mode() {
    let test_dir = TestDir::new();
    let output =
        test_dir.test_command_success_and_get_output(["compare", "--quiet", ".env1", ".env2"]);
    let unexpected_output = String::from("Nothing to compare\n");

    assert_ne!(output, unexpected_output);
}
