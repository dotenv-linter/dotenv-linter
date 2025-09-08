use crate::common::*;

#[test]
fn extra_blank_line() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF\n\n\nFOO=BAR\n");
    let expected_output = fix_output(&[(
        ".env",
        &[".env:3 ExtraBlankLine: Extra blank line detected"],
    )]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile.contents().as_str(), "ABC=DEF\n\nFOO=BAR\n");

    testdir.close();
}

#[test]
fn extra_blank_line_with_control_comments() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(
        ".env",
        "FOO=BAR\n\n# dotenv-linter:off ExtraBlankLine\n\n\n# dotenv-linter:on \
         ExtraBlankLine\nBAR=FOO\n\n\n",
    );
    let expected_output = fix_output(&[(
        ".env",
        &[
            ".env:9 ExtraBlankLine: Extra blank line detected",
            ".env:10 ExtraBlankLine: Extra blank line detected",
        ],
    )]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "FOO=BAR\n\n# dotenv-linter:off ExtraBlankLine\n\n\n# dotenv-linter:on \
         ExtraBlankLine\nBAR=FOO\n"
    );

    testdir.close();
}
