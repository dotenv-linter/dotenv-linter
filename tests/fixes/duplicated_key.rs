use crate::common::TestDir;

#[test]
fn duplicated_key() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF\nABC=XYZ\nFOO=BAR\nFOO=BAZ\n");
    let expected_output = String::from(
        "Fixed warnings:\n\
        .env:2 DuplicatedKey: The ABC key is duplicated\n\
        .env:4 DuplicatedKey: The FOO key is duplicated\n",
    );
    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "ABC=DEF\n# ABC=XYZ\nFOO=BAR\n# FOO=BAZ\n"
    );

    testdir.close();
}
