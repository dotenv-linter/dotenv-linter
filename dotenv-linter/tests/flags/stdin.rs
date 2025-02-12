use crate::common::*;

#[test]
fn check_output_in_stdin_mode_success() {
    let test_dir = TestDir::new();
    let testfile_to_check = test_dir.create_testfile(".env", "BAR=Baz\n");

    let expected_output = format!(
        "Checking {a}

No problems found
",
        a = ""
    );

    test_dir.test_command_success_with_args_and_stdin_input(
        with_default_args(&[]),
        testfile_to_check,
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

    test_dir.test_command_failed_with_args_and_stdin_input(
        with_default_args(&[]),
        testfile_to_check,
        expected_output,
    );
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

    test_dir.test_command_success_with_args_and_stdin_input(
        with_default_args(&[]),
        testfile_to_check,
        expected_output,
    );
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
