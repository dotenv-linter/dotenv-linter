use crate::common::*;
use std::env;

fn new_version_output() -> String {
    format!(
        "A new release of dotenv-linter is available: v3.0.0 -> v3.1.1\n\
        https://github.com/dotenv-linter/dotenv-linter/releases/tag/v3.1.1"
    )
}

#[test]
fn print_new_version() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "FOO=bar\n");
    let expected_output = check_output(&[(".env", &[])]);
    let expected_output = format!("{}\n{}\n", expected_output, new_version_output());

    let args: &[&str; 0] = &[];
    test_dir.test_command_success_with_args(args, expected_output);
}

#[test]
fn print_new_version_if_nothing_to_check() {
    let test_dir = TestDir::new();
    let expected_output = format!("Nothing to check\n\n{}\n", new_version_output());

    let args: &[&str; 0] = &[];
    test_dir.test_command_success_with_args(args, expected_output);
}

#[test]
fn do_not_print_new_version() {
    env::set_var("DOTENV_LINTER_NOT_CHECK_UPDATES", "true");
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "FOO=bar\n");
    let expected_output = check_output(&[(".env", &[])]);
    let expected_output = format!("{}", expected_output);

    let args: &[&str; 0] = &[];
    test_dir.test_command_success_with_args(args, expected_output);

    env::remove_var("DOTENV_LINTER_NOT_CHECK_UPDATES");
}
