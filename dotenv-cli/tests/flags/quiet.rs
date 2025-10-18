use crate::common::*;

#[test]
fn check_output_in_quiet_mode() {
    let test_dir = TestDir::new();
    let testfile_to_check = test_dir.create_testfile(".env", " BAR='Baz'\n");

    let args = &["check", ".", "--quiet"];
    let expected_output = format!(
        "{a}:1 LeadingCharacter: Invalid leading character detected\n{a}:1 QuoteCharacter: The \
         value has quote characters (\', \")\n",
        a = testfile_to_check.shortname_as_str()
    );

    test_dir.test_command_fail_with_args(with_default_args(args), expected_output);
}

#[test]
fn check_output_for_multiple_files_in_quiet_mode() {
    let test_dir = TestDir::new();
    let testfile_1 = test_dir.create_testfile(".env", "BAR='Baz'\n");
    let testfile_2 = test_dir.create_testfile(".env2", " BAR=\n");

    let args = &["check", ".", "--quiet"];
    let expected_output = format!(
        "{a}:1 QuoteCharacter: The value has quote characters (\', \")\n{b}:1 LeadingCharacter: \
         Invalid leading character detected\n",
        a = testfile_1.shortname_as_str(),
        b = testfile_2.shortname_as_str()
    );

    test_dir.test_command_fail_with_args(with_default_args(args), expected_output);
}

#[test]
fn fix_output_in_quiet_mode() {
    let test_dir = TestDir::new();
    let _ = test_dir.create_testfile(".env", "abc=DEF\n\nF=BAR\nB=bbb\n");

    let args = &["check", ".", "--quiet"];
    let expected_output = format!("\nAll warnings are fixed. Total: {}\n", 2);

    test_dir.test_command_fix_success_with_args(expected_output, args);
    test_dir.close();
}
