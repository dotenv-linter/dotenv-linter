use crate::common::*;

#[test]
fn trailing_whitespace() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF \n\nFOO=BAR   \n");
    let expected_output = fix_output(&[
        ".env:1 TrailingWhitespace: Trailing whitespace detected",
        ".env:3 TrailingWhitespace: Trailing whitespace detected",
    ]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile.contents().as_str(), "ABC=DEF\n\nFOO=BAR\n");

    testdir.close();
}
