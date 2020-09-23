use crate::common::TestDir;

mod duplicated_key;
mod ending_blank_line;
mod extra_blank_line;
mod incorrect_delimiter;
mod key_without_value;
mod leading_character;
mod lowercase_key;
mod quote_character;
mod space_character;
mod trailing_whitespace;
mod unordered_key;

#[test]
fn correct_file() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF\nD=BAR\n\nFOO=BAR\n");

    testdir.test_command_fix_success(String::new());

    assert_eq!(testfile.contents().as_str(), "ABC=DEF\nD=BAR\n\nFOO=BAR\n");

    testdir.close();
}

#[test]
fn multiple_files() {
    let testdir = TestDir::new();

    let testfile1 = testdir.create_testfile("1.env", "AB=DEF\nD=BAR\n\nF=BAR\n");
    let testfile2 = testdir.create_testfile("2.env", "abc=DEF\n\nF=BAR\nB=bbb\n");
    let testfile3 = testdir.create_testfile("3.env", "A=b \nab=DEF\n\nA=c\n");

    let expected_output = String::from(
        "Fixed warnings:\n\
        2.env:1 LowercaseKey: The abc key should be in uppercase\n\
        2.env:4 UnorderedKey: The B key should go before the F key\n\
        3.env:1 TrailingWhitespace: Trailing whitespace detected\n\
        3.env:2 LowercaseKey: The ab key should be in uppercase\n\
        3.env:4 DuplicatedKey: The A key is duplicated\n",
    );
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile1.contents().as_str(), "AB=DEF\nD=BAR\n\nF=BAR\n");
    assert_eq!(testfile2.contents().as_str(), "ABC=DEF\n\nB=bbb\nF=BAR\n");
    assert_eq!(testfile3.contents().as_str(), "A=b\nAB=DEF\n\n# A=c\n");

    testdir.close();
}
