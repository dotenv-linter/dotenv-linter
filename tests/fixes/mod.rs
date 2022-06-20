use crate::common::*;
use std::ffi::OsStr;
use std::fs;

mod duplicated_key;
mod ending_blank_line;
mod extra_blank_line;
mod incorrect_delimiter;
mod key_without_value;
mod leading_character;
mod lowercase_key;
mod quote_character;
mod space_character;
mod substitution_key;
mod trailing_whitespace;
mod unordered_key;
mod value_without_quotes;

#[test]
fn correct_file() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF\nD=BAR\n\nFOO=BAR\n");

    testdir.test_command_fix_success(fix_output(&[(".env", &[])]));

    assert_eq!(testfile.contents().as_str(), "ABC=DEF\nD=BAR\n\nFOO=BAR\n");

    testdir.close();
}

#[test]
fn skip_checks() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "A1=1\nA2=2\na0=0\na2=2\n");

    let expected_output = fix_output(&[(
        ".env",
        &[
            ".env:3 LowercaseKey: The a0 key should be in uppercase",
            ".env:4 LowercaseKey: The a2 key should be in uppercase",
        ],
    )]);

    testdir.test_command_fix_success_with_args(
        expected_output,
        &["--skip", "DuplicatedKey", "UnorderedKey"],
    );

    assert_eq!(testfile.contents().as_str(), "A1=1\nA2=2\nA0=0\nA2=2\n");

    testdir.close();
}

#[test]
fn multiple_files() {
    let testdir = TestDir::new();

    let testfile1 = testdir.create_testfile("1.env", "AB=DEF\nD=BAR\n\nF=BAR\n");
    let testfile2 = testdir.create_testfile("2.env", "abc=DEF\n\nF=BAR\nB=bbb\n");
    let testfile3 = testdir.create_testfile("3.env", "A=b \nab=DEF\n\nA=c\n");

    let expected_output = fix_output(&[
        ("1.env", &[]),
        (
            "2.env",
            &[
                "2.env:1 LowercaseKey: The abc key should be in uppercase",
                "2.env:4 UnorderedKey: The B key should go before the F key",
            ],
        ),
        (
            "3.env",
            &[
                "3.env:1 TrailingWhitespace: Trailing whitespace detected",
                "3.env:2 LowercaseKey: The ab key should be in uppercase",
                "3.env:4 DuplicatedKey: The A key is duplicated",
            ],
        ),
    ]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile1.contents().as_str(), "AB=DEF\nD=BAR\n\nF=BAR\n");
    assert_eq!(testfile2.contents().as_str(), "ABC=DEF\n\nB=bbb\nF=BAR\n");
    assert_eq!(testfile3.contents().as_str(), "A=b\nAB=DEF\n\n# A=c\n");

    testdir.close();
}

#[test]
fn fixtures() {
    // Iterate through *.env files in the fixture directory
    // and compare them with corresponding *.env.golden files
    for mut fixture_path in fs::read_dir("tests/fixes/fixtures")
        .expect("list dir")
        .filter_map(|res| {
            let path = res.expect("dir entry").path();
            if path.extension() == Some(OsStr::new("env")) {
                Some(path)
            } else {
                None
            }
        })
    {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(
            ".env",
            fs::read_to_string(fixture_path.as_path())
                .expect("fixture file")
                .as_str(),
        );

        testdir.test_command_fix_success_without_output();

        fixture_path.set_extension("env.golden");
        let expected_content = fs::read_to_string(fixture_path.as_path()).expect("golden file");

        assert_eq!(testfile.contents(), expected_content);

        // Check the fixed file again and then clean up
        testdir
            .test_command_success_with_args(with_default_args(&[]), check_output(&[(".env", &[])]));
    }
}
