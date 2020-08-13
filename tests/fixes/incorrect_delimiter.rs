use crate::common::TestDir;

#[test]
fn incorrect_delimiter() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "RAILS-ENV=development\n\nSECRET,KEY*=xyz\n");
    let expected_output = String::from(
        "Fixed warnings:\n\
        .env:1 IncorrectDelimiter: The RAILS-ENV key has incorrect delimiter\n\
        .env:3 IncorrectDelimiter: The SECRET,KEY* key has incorrect delimiter\n",
    );
    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "RAILS_ENV=development\n\nSECRET_KEY_=xyz\n"
    );

    testdir.close();
}
