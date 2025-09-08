use crate::common::*;

#[test]
fn correct_files() {
    let contents = [
        "A=\"B B\"\nF=\"BAR B\"\nFOO=\"BAR BAZ\"\n",
        "A=\"B B\"\r\nF=\"BAR B\"\r\nFOO=\"BAR BAZ\"\r\n",
        "# comment\nABC=\"DEF GHI\"\n",
    ];

    for content in contents {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &[testfile.as_str()];

        let expected_output = check_output(&[(".env", &[])]);

        testdir.test_command_success_with_args(with_default_args(args), expected_output);
    }
}

#[test]
fn incorrect_files() {
    let contents = [
        "A=\"B B\"\nF=\"BAR B\"\nFOO=BAR BAZ\n",
        "A=\"B B\"\r\nF=BAR B\r\nFOO=\"BAR BAZ\"\r\n",
        "# comment\nABC=DEF GHI\n",
    ];
    let expected_line_numbers = [3, 2, 2];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &[testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[format!(
                ".env:{} ValueWithoutQuotes: This value needs to be surrounded in quotes",
                expected_line_numbers[i]
            )
            .as_str()],
        )]);

        testdir.test_command_fail_with_args(with_default_args(args), expected_output);
    }
}

#[test]
fn multiline_value() {
    let content = "FOO=\"new\\nline value\"\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &[testfile.as_str()];
    let expected_output = check_output(&[(".env", &[])]);

    testdir.test_command_success_with_args(with_default_args(args), expected_output);
}
