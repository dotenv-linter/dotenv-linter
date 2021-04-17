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
        "1BAR=BAZ\nFOO=TEST\n",
        "-BAR=BAZ\nA=B\n",
        ".BAR=BAZ\n",
        "\tA=B\nFOO=BAZ\n",
    ];
    let expected_line_numbers = vec![1, 1, 1, 1];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &[testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[format!(
                ".env:{} LeadingCharacter: Invalid leading character detected",
                expected_line_numbers[i]
            )
            .as_str()],
        )]);

        testdir.test_command_fail_with_args(args, expected_output);
    }
}
