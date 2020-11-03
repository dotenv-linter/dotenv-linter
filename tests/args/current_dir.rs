use crate::common::*;

#[test]
fn exits_with_0_on_no_warnings() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "FOO=bar\n");
    let expected_output = check_output(&[(".env", &[])]);
    test_dir.test_command_success(expected_output);
}

#[test]
fn checks_current_dir() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "FOO\n");

    testdir.test_command_fail(check_output(&[(
        testfile.shortname_as_str(),
        &[format!(
            "{}:1 KeyWithoutValue: The FOO key should be with a value or have an equal sign",
            testfile.shortname_as_str()
        )
        .as_str()],
    )]));
}

#[test]
fn checks_current_dir_with_dot_arg() {
    let testdir = TestDir::new();
    testdir.create_testfile("test.env", "foo=\n");

    let args = &["."];
    let expected_output = check_output(&[(
        "test.env",
        &["test.env:1 LowercaseKey: The foo key should be in uppercase"],
    )]);

    testdir.test_command_fail_with_args(args, expected_output);
}
