use crate::common::*;

#[test]
fn quote_character() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=\"DEF\"\n\nFOO=\'B\"AR\'\n");
    let expected_output = fix_output(&[(
        ".env",
        &[
            ".env:1 QuoteCharacter: The value has quote characters (\', \")",
            ".env:3 QuoteCharacter: The value has quote characters (\', \")",
        ],
    )]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile.contents().as_str(), "ABC=DEF\n\nFOO=BAR\n");

    testdir.close();
}
