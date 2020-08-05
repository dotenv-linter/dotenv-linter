use crate::common::TestDir;

#[test]
fn exclude_one_file() {
    let test_dir = TestDir::new();
    let testfile = test_dir.create_testfile(".env", " FOO=\n");
    test_dir.test_command_success_with_args(&["--exclude", testfile.as_str()]);
}

#[test]
fn exclude_two_files() {
    let test_dir = TestDir::new();
    let testfile_1 = test_dir.create_testfile(".env", " FOO=\n");
    let testfile_2 = test_dir.create_testfile(".loacl.env", " BAR=\n");

    test_dir.test_command_success_with_args(&[
        "-e",
        testfile_1.as_str(),
        "-e",
        testfile_2.as_str(),
    ]);
}

#[test]
fn exclude_one_file_check_one_file() {
    let test_dir = TestDir::new();
    let testfile_to_check = test_dir.create_testfile(".env", " FOO=\n");
    let testfile_to_exclude = test_dir.create_testfile(".exclude-me.env", " BAR=\n");

    let args = &["--exclude", testfile_to_exclude.as_str()];
    let expected_output = format!(
        "{}:1 LeadingCharacter: Invalid leading character detected\n\nFound 1 problem\n",
        testfile_to_check.shortname_as_str()
    );

    test_dir.test_command_fail_with_args(args, expected_output);
}
