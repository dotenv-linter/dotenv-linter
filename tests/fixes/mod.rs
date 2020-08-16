use crate::common::TestDir;

mod ending_blank_line;
mod space_character;
mod trailing_whitespace;

#[test]
fn correct_file() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF\nD=BAR\n\nFOO=BAR\n");

    testdir.test_command_fix_success(String::new());

    assert_eq!(testfile.contents().as_str(), "ABC=DEF\nD=BAR\n\nFOO=BAR\n");

    testdir.close();
}

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

#[test]
fn key_without_value() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "FOO\n\nBAR=\n\nBAZ=QUX\n");
    let expected_output = String::from(
        "Fixed warnings:\n\
        .env:1 KeyWithoutValue: The FOO key should be with a value or have an equal sign\n",
    );
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile.contents().as_str(), "FOO=\n\nBAR=\n\nBAZ=QUX\n");

    testdir.close();
}

#[test]
fn unfixed_warnings() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "A=DEF\nB=BAR \nf=BAR\n\n");

    let expected_output = String::from(
        "Fixed warnings:\n\
        .env:2 TrailingWhitespace: Trailing whitespace detected\n\
        .env:3 LowercaseKey: The f key should be in uppercase\n\
        \n\
        Unfixed warnings:\n\
        .env:5 ExtraBlankLine: Extra blank line detected\n",
    );
    testdir.test_command_fix_fail(expected_output);

    assert_eq!(testfile.contents().as_str(), "A=DEF\nB=BAR\nF=BAR\n\n");

    testdir.close();
}

#[test]
fn multiple_files() {
    let testdir = TestDir::new();

    let testfile1 = testdir.create_testfile("1.env", "AB=DEF\nD=BAR\n\nF=BAR\n");
    let testfile2 = testdir.create_testfile("2.env", "abc=DEF\n\nF=BAR\n");
    let testfile3 = testdir.create_testfile("3.env", "A=b \nab=DEF\n\n");

    let expected_output = String::from(
        "Fixed warnings:\n\
        2.env:1 LowercaseKey: The abc key should be in uppercase\n\
        3.env:1 TrailingWhitespace: Trailing whitespace detected\n\
        3.env:2 LowercaseKey: The ab key should be in uppercase\n\
        \n\
        Unfixed warnings:\n\
        3.env:4 ExtraBlankLine: Extra blank line detected\n",
    );
    testdir.test_command_fix_fail(expected_output);

    assert_eq!(testfile1.contents().as_str(), "AB=DEF\nD=BAR\n\nF=BAR\n");
    assert_eq!(testfile2.contents().as_str(), "ABC=DEF\n\nF=BAR\n");
    assert_eq!(testfile3.contents().as_str(), "A=b\nAB=DEF\n\n");

    testdir.close();
}
