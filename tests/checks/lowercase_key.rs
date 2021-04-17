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
    let contents = vec!["FOO=TEST\nbar=hello\n", "BAR_FoO=hello\nFOO=TEST\n"];
    let expected_line_numbers = vec![2, 1];
    let expected_variable_names = vec!["bar", "BAR_FoO"];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &[testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[format!(
                ".env:{} LowercaseKey: The {} key should be in uppercase",
                expected_line_numbers[i], expected_variable_names[i],
            )
            .as_str()],
        )]);

        testdir.test_command_fail_with_args(args, expected_output);
    }
}
