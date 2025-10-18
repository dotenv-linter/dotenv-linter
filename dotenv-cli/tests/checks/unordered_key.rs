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
    let contents = ["B=3\nA=2\n", "A=value\nFOO=TEST\nBAR=45\n"];
    let expected = [(2, "A", "B"), (3, "BAR", "FOO")];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &["check", testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[format!(
                ".env:{} UnorderedKey: The {} key should go before the {} key",
                expected[i].0, expected[i].1, expected[i].2,
            )
            .as_str()],
        )]);

        testdir.test_command_fail_with_args(with_default_args(args), expected_output);
    }
}

#[test]
fn many_incorrect_variables() {
    let content = "A=B\nFOO=TEST\nBAZ=value\nBAR=45\nZERO=0\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &["check", testfile.as_str()];
    let expected_output = check_output(&[(
        ".env",
        &[
            ".env:3 UnorderedKey: The BAZ key should go before the FOO key",
            ".env:4 UnorderedKey: The BAR key should go before the BAZ key",
        ],
    )]);

    testdir.test_command_fail_with_args(with_default_args(args), expected_output);
}
