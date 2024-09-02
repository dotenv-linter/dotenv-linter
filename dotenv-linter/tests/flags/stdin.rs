use crate::common::*;

#[test]
fn check_output_in_stdin_mode_no_file_name_success() {
    let test_dir = TestDir::new();
    let testfile_to_check = test_dir.create_testfile(".env", "BAR=Baz\n");

    let expected_output = format!(
        "Checking {a}

No problems found
",
        a = ""
    );

    test_dir.test_command_check_success_with_stdin_input(testfile_to_check, expected_output);
}

#[test]
fn check_output_in_stdin_mode_with_file_name_success() {
    let test_dir = TestDir::new();
    let filename = ".env";
    let testfile_to_check = test_dir.create_testfile(filename, "BAR=Baz\n");

    let expected_output = format!(
        "Checking {a}

No problems found
",
        a = testfile_to_check.shortname_as_str()
    );

    test_dir.test_command_check_success_with_stdin_input_and_filename(
        testfile_to_check,
        filename,
        expected_output,
    );
}

#[test]
fn check_output_in_stdin_mode_failure() {
    let test_dir = TestDir::new();
    let testfile_to_check = test_dir.create_testfile(".env", "BAR='Baz'\n");

    let expected_output = format!(
        "Checking \n{a}:1 QuoteCharacter: The value has quote characters (\', \")\n\nFound 1 problem\n",
        a = ""
    );

    test_dir.test_command_check_failed_with_stdin_input(testfile_to_check, expected_output);
}

#[test]
fn check_output_in_stdin_mode_empty_file() {
    let test_dir = TestDir::new();
    let testfile_to_check = test_dir.create_testfile(".env", "");

    let expected_output = format!(
        "Checking {a}

No problems found
",
        a = ""
    );

    test_dir.test_command_check_success_with_stdin_input(testfile_to_check, expected_output);
}

#[test]
fn fix_output_in_stdin_mode_problems() {
    let test_dir = TestDir::new();
    let test_file = test_dir.create_testfile(".env", "RAILS_ENV=development\n\nSECRET,KEY*=xyz\n");

    let expected_output = format!(
        "RAILS_ENV=development

SECRET_KEY_=xyz
"
    );

    test_dir.test_command_fix_success_with_stdin_input(&test_file, expected_output);
}

#[test]
fn fix_output_in_stdin_mode_no_problems() {
    let test_dir = TestDir::new();
    let test_file = test_dir.create_testfile(".env", "RAILS_ENV=development\n\nSECRET_KEY_=xyz\n");

    let expected_output = format!(
        "RAILS_ENV=development

SECRET_KEY_=xyz
"
    );

    test_dir.test_command_fix_success_with_stdin_input(&test_file, expected_output);
}

#[test]
fn fix_output_in_stdin_mode_empty_file() {
    let test_dir = TestDir::new();
    let test_file = test_dir.create_testfile(".env", "");

    let expected_output = "";

    test_dir.test_command_fix_success_with_stdin_input(&test_file, expected_output);
}
