use crate::common::*;

#[test]
fn ending_blank_line() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF\nFOO=BAR");
    let expected_output =
        fix_output(&[".env:2 EndingBlankLine: No blank line at the end of the file"]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile.contents().as_str(), "ABC=DEF\nFOO=BAR\n");

    testdir.close();
}
