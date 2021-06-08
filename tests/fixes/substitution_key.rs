use crate::common::*;

#[test]
fn substitution_key() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(
        ".env",
        "ABC=${BAR$XYZ}\nBYZ=${BAR!}\n\nFOO=${ABC-$BAR}\nGOD=${ENVS${BAR}\nXYZ=\\$BAR}\n",
    );
    let expected_output = fix_output(&[(
        ".env",
        &[
            ".env:1 SubstitutionKey: The ABC key is not assigned properly",
            ".env:2 SubstitutionKey: The BYZ key is not assigned properly",
            ".env:4 SubstitutionKey: The FOO key is not assigned properly",
            ".env:5 SubstitutionKey: The GOD key is not assigned properly",
        ],
    )]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "ABC=${BAR}${XYZ}\nBYZ=${BAR}!}\n\nFOO=${ABC}-${BAR}\nGOD=${ENVS}${BAR}\nXYZ=\\$BAR}\n"
    );

    testdir.close();
}
