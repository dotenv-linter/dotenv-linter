use crate::common::*;

#[test]
fn correct_files() {
    let contents = vec![
        "A=B\nF=BAR\n\nFOO=BAR\n",
        "A=B\r\nF=BAR\r\n\r\nFOO=BAR\r\n",
        "\n# comment\n\nABC=DEF\n",
    ];

    for content in contents {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &[testfile.as_str()];

        testdir.test_command_success_with_args(args);
    }
}

#[test]
fn two_blank_lines_at_the_beginning() {
    let content = "\n\nABC=DEF\nD=BAR\nFOO=BAR\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &[testfile.as_str()];
    let expected_output = check_output(&[".env:2 ExtraBlankLine: Extra blank line detected"]);

    testdir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn two_blank_lines_in_the_middle() {
    let content = "ABC=DEF\nD=BAR\n\n\nFOO=BAR\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &[testfile.as_str()];
    let expected_output = check_output(&[".env:4 ExtraBlankLine: Extra blank line detected"]);

    testdir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn two_blank_lines_at_the_end() {
    let content = "ABC=DEF\nD=BAR\nFOO=BAR\n\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &[testfile.as_str()];
    let expected_output = check_output(&[".env:5 ExtraBlankLine: Extra blank line detected"]);

    testdir.test_command_fail_with_args(args, expected_output);
}
