use crate::common::TestDir;

#[test]
fn exits_with_0_on_no_warnings() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "FOO=bar\n");
    test_dir.test_command_success();
}

#[test]
fn checks_current_dir() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "FOO\n");

    testdir.test_command_fail(
        format!(
            "{}:1 KeyWithoutValue: The FOO key should be with a value or have an equal sign\n\nFound 1 problem\n",
            testfile.shortname_as_str()
        )
    );
}

#[test]
fn checks_current_dir_with_dot_arg() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile("test.env", "foo=\n");

    let args = &["."];
    let expected_output = format!(
        "{}:1 LowercaseKey: The foo key should be in uppercase\n\nFound 1 problem\n",
        testfile.shortname_as_str(),
    );

    testdir.test_command_fail_with_args(args, expected_output);
}
