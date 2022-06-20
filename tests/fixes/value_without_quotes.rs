use crate::common::*;

#[test]
fn value_without_quotes() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF GHI\nFOO=BAR BAZ\n");
    let expected_output = fix_output(&[(
        ".env",
        &[
            ".env:1 ValueWithoutQuotes: This value needs to be surrounded in quotes",
            ".env:2 ValueWithoutQuotes: This value needs to be surrounded in quotes",
        ],
    )]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "ABC=\"DEF GHI\"\nFOO=\"BAR BAZ\"\n"
    );

    testdir.close();
}
