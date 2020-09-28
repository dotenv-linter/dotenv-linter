use crate::common::*;

#[test]
fn space_character() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC = DEF\n\nFOO= BAR\n");
    let expected_output = fix_output(&[
        ".env:1 SpaceCharacter: The line has spaces around equal sign",
        ".env:3 SpaceCharacter: The line has spaces around equal sign",
    ]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile.contents().as_str(), "ABC=DEF\n\nFOO=BAR\n");

    testdir.close();
}
