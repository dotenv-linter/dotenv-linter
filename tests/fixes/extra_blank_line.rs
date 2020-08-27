use crate::common::TestDir;

#[test]
fn extra_blank_line() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF\n\n\nFOO=BAR\n");
    let expected_output = String::from(
        "Fixed warnings:\n\
        .env:3 ExtraBlankLine: Extra blank line detected\n",
    );
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile.contents().as_str(), "ABC=DEF\n\nFOO=BAR\n");

    testdir.close();
}
