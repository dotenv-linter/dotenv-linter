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
    let contents = vec!["FOO=TEST\nFOO-BAR=BAZ\n", "A=B\nFOO-*-BAR=BAZ\n"];
    let expected_line_numbers = vec![2, 2];
    let expected_variable_names = vec!["FOO-BAR", "FOO-*-BAR"];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &[testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[format!(
                ".env:{} IncorrectDelimiter: The {} key has incorrect delimiter",
                expected_line_numbers[i], expected_variable_names[i],
            )
            .as_str()],
        )]);

        testdir.test_command_fail_with_args(args, expected_output);
    }
}

#[test]
fn many_incorrect_variables() {
    let content = "A=B\nBAZ*-KEY=test\nFOO-BAR=BAZ\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &[testfile.as_str()];
    let expected_output = check_output(&[(
        ".env",
        &[
            ".env:2 IncorrectDelimiter: The BAZ*-KEY key has incorrect delimiter",
            ".env:3 IncorrectDelimiter: The FOO-BAR key has incorrect delimiter",
        ],
    )]);

    testdir.test_command_fail_with_args(args, expected_output);
}
