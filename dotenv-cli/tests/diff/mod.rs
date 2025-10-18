use crate::common::TestDir;

#[test]
fn files_with_same_environment_variables() {
    let test_dir = TestDir::new();
    let testfile_one = test_dir.create_testfile(".env1", "FOO=abc\nBAR=def");
    let testfile_two = test_dir.create_testfile(".env2", "FOO=abc\nBAR=def");
    let expected_output = "Comparing .env1\nComparing .env2\nNo difference found\n";

    test_dir.test_command_success_with_args(
        ["diff", testfile_one.as_str(), testfile_two.as_str()],
        expected_output,
    );
}

#[test]
fn files_with_same_environment_variables_in_quiet_mode() {
    let test_dir = TestDir::new();
    let testfile_one = test_dir.create_testfile(".env1", "FOO=abc\nBAR=def");
    let testfile_two = test_dir.create_testfile(".env2", "FOO=abc\nBAR=def");
    let expected_output = "";

    test_dir.test_command_success_with_args(
        [
            "diff",
            "--quiet",
            testfile_one.as_str(),
            testfile_two.as_str(),
        ],
        expected_output,
    );
}

#[test]
fn files_with_different_environment_variables() {
    let test_dir = TestDir::new();
    let testfile_one = test_dir.create_testfile(".env1", "FOO=abc");
    let testfile_two = test_dir.create_testfile(".env2", "FOO=abc\nBAR=def");
    let expected_output = "Comparing .env1\nComparing .env2\n.env1 is missing keys: BAR\n";

    test_dir.test_command_fail_with_args(
        ["diff", testfile_one.as_str(), testfile_two.as_str()],
        expected_output,
    )
}

#[test]
fn files_with_different_environment_variables_in_quiet_mode() {
    let test_dir = TestDir::new();
    let testfile_one = test_dir.create_testfile(".env1", "FOO=abc");
    let testfile_two = test_dir.create_testfile(".env2", "FOO=abc\nBAR=def");
    let expected_output = ".env1 is missing keys: BAR\n";

    test_dir.test_command_fail_with_args(
        [
            "diff",
            "--quiet",
            testfile_one.as_str(),
            testfile_two.as_str(),
        ],
        expected_output,
    )
}
