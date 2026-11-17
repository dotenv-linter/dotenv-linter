use crate::common::*;

#[test]
fn substitution_key() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(
        ".env",
        "ABC=${BAR$XYZ}\nBYZ=${BAR!}\n\nFOO=${ABC-$BAR}\nGOD=${ENVS${BAR}\nXYZ=\\$BAR}\n",
    );
    let expected_output = "\
Fixing .env
Could not fix all warnings
.env:1 SubstitutionKey: The ABC key is not assigned properly
.env:2 SubstitutionKey: The BYZ key is not assigned properly
.env:4 SubstitutionKey: The FOO key is not assigned properly
.env:5 SubstitutionKey: The GOD key is not assigned properly
";
    testdir.test_command_fix_fail(expected_output);

    // `${ENVS${BAR}` has no closing brace for the outer substitution, so it is
    // split into two valid ones.
    assert_eq!(
        testfile.contents().as_str(),
        "ABC=${BAR$XYZ}\nBYZ=${BAR!}\n\nFOO=${ABC-$BAR}\nGOD=${ENVS}${BAR}\nXYZ=\\$BAR}\n"
    );

    testdir.close();
}

#[test]
fn substitution_key_fixed() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "GOD=${ENVS${BAR}\n");

    let expected_output = fix_output(&[(
        ".env",
        &[".env:1 SubstitutionKey: The GOD key is not assigned properly"],
    )]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(testfile.contents().as_str(), "GOD=${ENVS}${BAR}\n");

    testdir.close();
}

#[test]
fn substitution_key_partially_fixed() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "URL=${FOO-BAR}://$HOST-db\n");

    let expected_fix_output = "\
Fixing .env
Could not fix all warnings
.env:1 SubstitutionKey: The URL key is not assigned properly
";
    testdir.test_command_fix_fail(expected_fix_output);

    // The safe part is fixed and the text after the ambiguous substitution is
    // preserved, but the warning itself is still there.
    assert_eq!(
        testfile.contents().as_str(),
        "URL=${FOO-BAR}://${HOST}-db\n"
    );

    let expected_check_output = check_output(&[(
        ".env",
        &[".env:1 SubstitutionKey: The URL key is not assigned properly"],
    )]);
    testdir.test_command_fail_with_args(with_default_args(&["check", "."]), expected_check_output);
}
