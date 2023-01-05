use crate::common::*;

#[test]
fn incorrect_delimiter() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "RAILS-ENV=development\n\nSECRET,KEY*=xyz\n");
    let expected_output = fix_output(&[(
        ".env",
        &[
            ".env:1 IncorrectDelimiter: The RAILS-ENV key has incorrect delimiter",
            ".env:3 IncorrectDelimiter: The SECRET,KEY* key has incorrect delimiter",
        ],
    )]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "RAILS_ENV=development\n\nSECRET_KEY_=xyz\n"
    );

    testdir.close();
}
