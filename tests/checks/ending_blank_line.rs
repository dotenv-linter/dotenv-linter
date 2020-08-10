use crate::common::TestDir;

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

        testdir.test_command_success_with_args(args);
    }
}

#[test]
fn incorrect_files() {
    let contents = vec![
        "ABC=DEF\nD=BAR\nFOO=BAR",
        "C=D\r\nK=L\r\nX=Y",
        "A=B",
        "# Comment 1\n# Comment 2\n# Comment 3",
    ];
    let expected_line_numbers = vec![3, 3, 1, 3];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &[testfile.as_str()];
        let expected_output = format!(
            "{}:{} EndingBlankLine: No blank line at the end of the file\n\nFound 1 problem\n",
            testfile.shortname_as_str(),
            expected_line_numbers[i]
        );

        testdir.test_command_fail_with_args(args, expected_output);
    }
}
