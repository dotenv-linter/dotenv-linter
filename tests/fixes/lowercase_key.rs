use crate::common::TestDir;

#[test]
fn lowercase_key() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "abc=DEF\n\nfOO=BAR\n");
    let expected_output = String::from(
        "Fixed warnings:\n\
        .env:1 LowercaseKey: The abc key should be in uppercase\n\
        .env:3 LowercaseKey: The fOO key should be in uppercase\n",
    );
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile.contents().as_str(), "ABC=DEF\n\nFOO=BAR\n");

    testdir.close();
}
