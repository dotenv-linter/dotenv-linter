use crate::common::*;

#[test]
fn duplicated_key() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF\nABC=XYZ\nFOO=BAR\nFOO=BAZ\n");
    let expected_output = fix_output(&[(
        ".env",
        &[
            ".env:2 DuplicatedKey: The ABC key is duplicated",
            ".env:4 DuplicatedKey: The FOO key is duplicated",
        ],
    )]);

    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "ABC=DEF\n# ABC=XYZ\nFOO=BAR\n# FOO=BAZ\n"
    );

    testdir.close();
}

#[test]
fn duplicated_key_with_multiline_value() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(
        ".env",
        "KEY=\"-----BEGIN-----\nabc\n-----END-----\"\n\
         KEY=\"-----BEGIN-----\nxyz\n-----END-----\"\n",
    );
    let expected_output = fix_output(&[(
        ".env",
        &[".env:4 DuplicatedKey: The KEY key is duplicated"],
    )]);

    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "KEY=\"-----BEGIN-----\nabc\n-----END-----\"\n\
         # KEY=\"-----BEGIN-----\n# xyz\n# -----END-----\"\n"
    );

    testdir.test_command_success_with_args(
        with_default_args(&["check", "."]),
        check_output(&[(".env", &[])]),
    );
}
