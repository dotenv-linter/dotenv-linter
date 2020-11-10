use crate::common::*;

#[test]
fn correct_files() {
    let contents = vec![
        "A=B\nF=BAR\nFOO=BAR\n",
        "A=B\r\nF=BAR\r\nFOO=BAR\r\n",
        "# comment\nABC=DEF\n",
    ];

    for content in contents {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &[testfile.as_str()];

        let expected_output = check_output(&[(".env", &[])]);

        testdir.test_command_success_with_args(args, expected_output);
    }
}

#[test]
fn incorrect_files() {
    let contents = vec![
        "A=\"B\"\nF=BAR\nFOO=BAR\n",
        "A=B\r\nF=BAR\r\nFOO=\"BAR\"\r\n",
        "# comment\nABC=\"DEF\"\n",
    ];
    let expected_line_numbers = vec![1, 3, 2];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &[testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[format!(
                ".env:{} QuoteCharacter: The value has quote characters (', \")",
                expected_line_numbers[i]
            )
            .as_str()],
        )]);

        testdir.test_command_fail_with_args(args, expected_output);
    }
}

#[test]
fn multiline_value() {
    let content = "FOO=\"new\\nline\"\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &[testfile.as_str()];
    let expected_output = check_output(&[(".env", &[])]);

    testdir.test_command_success_with_args(args, expected_output);
}
