use crate::common::*;

#[test]
fn correct_files() {
    let contents = [
        "A=B\nF=BAR\nFOO=BAR\n",
        "A=B\r\nF=BAR\r\nFOO=BAR\r\n",
        "# comment\nABC=DEF\n",
    ];

    for content in contents {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &["check", testfile.as_str()];

        let expected_output = check_output(&[(".env", &[])]);

        testdir.test_command_success_with_args(with_default_args(args), expected_output);
    }
}

#[test]
fn incorrect_files() {
    let contents = ["FOO=TEST\nbar=hello\n", "BAR_FoO=hello\nFOO=TEST\n"];
    let expected = [(2, "bar"), (1, "BAR_FoO")];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &["check", testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[format!(
                ".env:{} LowercaseKey: The {} key should be in uppercase",
                expected[i].0, expected[i].1,
            )
            .as_str()],
        )]);

        testdir.test_command_fail_with_args(with_default_args(args), expected_output);
    }
}

#[test]
fn many_incorrect_variables() {
    let content = "FOO=TEST\nFoo_BAZ=BAR\nbar=TEST\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &["check", testfile.as_str()];
    let expected_output = check_output(&[(
        ".env",
        &[
            ".env:2 LowercaseKey: The Foo_BAZ key should be in uppercase",
            ".env:3 LowercaseKey: The bar key should be in uppercase",
        ],
    )]);

    testdir.test_command_fail_with_args(with_default_args(args), expected_output);
}
