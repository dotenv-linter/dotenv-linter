use crate::common::TestDir;

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
