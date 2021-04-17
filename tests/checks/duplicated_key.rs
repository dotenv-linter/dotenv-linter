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
    let contents = vec!["A=B\nF=BAR\nFOO=BAR\nFOO=BAZ\n", "A=BAR\nA=Foo\n"];
    let expected_line_numbers = vec![4, 2];
    let expected_variable_names = vec!["FOO", "A"];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &[testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[format!(
                ".env:{} DuplicatedKey: The {} key is duplicated",
                expected_line_numbers[i], expected_variable_names[i],
            )
            .as_str()],
        )]);

        testdir.test_command_fail_with_args(args, expected_output);
    }
}

#[test]
fn many_duplicates() {
    let content = "A=3\nA=84\nBAR=ABC\nBAR=Foo\nBAR=bazz\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &[testfile.as_str()];
    let expected_output = check_output(&[(
        ".env",
        &[
            ".env:2 DuplicatedKey: The A key is duplicated",
            ".env:4 DuplicatedKey: The BAR key is duplicated",
            ".env:5 DuplicatedKey: The BAR key is duplicated",
        ],
    )]);

    testdir.test_command_fail_with_args(args, expected_output);
}
