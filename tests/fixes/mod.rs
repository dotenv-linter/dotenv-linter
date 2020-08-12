mod quote_character;

use crate::common::TestDir;

mod duplicated_key;
mod ending_blank_line;
mod extra_blank_line;
mod incorrect_delimiter;
mod leading_character;
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
fn unordered_key() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(
        ".env",
        "# comment\n\
        \n\
        C=B\n\
        # A comment\n\
        A=B\n\
        X=Y\n\
        # more comments\n\
        \n\
        # middle comment\n\
        \n\
        M=123\n\
        # K comment\n\
        K=123\n\
        # I comment\n\
        # I comment\n\
        I=123\n\
        \n\
        A1=1\n\
        B1=1\n\
        \n\
        # end comment\n",
    );
    let expected_output = String::from(
        "Fixed warnings:\n\
        .env:5 UnorderedKey: The A key should go before the C key\n\
        .env:13 UnorderedKey: The K key should go before the M key\n\
        .env:16 UnorderedKey: The I key should go before the K key\n",
    );
    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "# comment\n\
        \n\
        # A comment\n\
        A=B\n\
        C=B\n\
        X=Y\n\
        # more comments\n\
        \n\
        # middle comment\n\
        \n\
        # I comment\n\
        # I comment\n\
        I=123\n\
        # K comment\n\
        K=123\n\
        M=123\n\
        \n\
        A1=1\n\
        B1=1\n\
        \n\
        # end comment\n",
    );

    testdir.close();
}

// We need to figure out how to test unfixed warnings now when all the fixers have been implemented
// (if required)

// #[test]
// fn unfixed_warnings() {
//     let testdir = TestDir::new();
//     let testfile = testdir.create_testfile(".env", "A=DEF\nB=BAR \nX-Y=Z\nf=BAR\nA=FOO\n");
//
//     let expected_output = String::from(
//         "Fixed warnings:\n\
//         .env:2 TrailingWhitespace: Trailing whitespace detected\n\
//         .env:3 IncorrectDelimiter: The X-Y key has incorrect delimiter\n\
//         .env:4 LowercaseKey: The f key should be in uppercase\n\
//         .env:5 DuplicatedKey: The A key is duplicated\n\
//         \n\
//         Unfixed warnings:\n\
//        .env:5 UnorderedKey: The A key should go before the A key\n",
//     );
//     testdir.test_command_fix_fail(expected_output);
//
//     assert_eq!(
//         testfile.contents().as_str(),
//         "A=DEF\nB=BAR\nX_Y=Z\nF=BAR\n# A=FOO\n"
//     );
//
//     testdir.close();
// }

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
