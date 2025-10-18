use crate::common::*;

#[test]
fn also_detect_lowercase_key() {
    let contents = vec!["test\n", "export test\n"];

    for contents in contents {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", contents);
        let args = &["check", testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[
                ".env:1 KeyWithoutValue: The test key should be with a value or have an equal sign",
                ".env:1 LowercaseKey: The test key should be in uppercase",
            ],
        )]);

        testdir.test_command_fail_with_args(with_default_args(args), expected_output);
    }
}
