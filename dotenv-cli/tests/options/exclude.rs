use crate::common::*;

#[test]
fn exclude_one_file() {
    let test_dir = TestDir::new();
    let testfile = test_dir.create_testfile(".env", " FOO=\n");

    let expected_output = r#"Nothing to check
"#;

    test_dir.test_command_success_with_args(
        with_default_args(&["check", ".", "--exclude", testfile.as_str()]),
        expected_output,
    );
}

#[test]
fn exclude_two_files() {
    let test_dir = TestDir::new();
    let testfile_1 = test_dir.create_testfile(".env", " FOO=\n");
    let testfile_2 = test_dir.create_testfile(".local.env", " BAR=\n");

    let expected_output = r#"Nothing to check
"#;

    test_dir.test_command_success_with_args(
        with_default_args(&[
            "check",
            ".",
            "-e",
            testfile_1.as_str(),
            "-e",
            testfile_2.as_str(),
        ]),
        expected_output,
    );
}

#[test]
fn exclude_one_file_check_one_file() {
    let test_dir = TestDir::new();
    let testfile_to_check = test_dir.create_testfile(".env", " FOO=\n");
    let testfile_to_exclude = test_dir.create_testfile(".exclude-me.env", " BAR=\n");

    let args = &["check", ".", "--exclude", testfile_to_exclude.as_str()];
    let expected_output = check_output(&[(
        testfile_to_check.shortname_as_str(),
        &[format!(
            "{}:1 LeadingCharacter: Invalid leading character detected",
            testfile_to_check.shortname_as_str()
        )
        .as_str()],
    )]);

    test_dir.test_command_fail_with_args(with_default_args(args), expected_output);
}
