use crate::common::TestDir;

#[test]
fn output_without_colors() {
    let test_dir = TestDir::new();
    let testfile_to_check = test_dir.create_testfile(".env", " BAR='Baz'\n");

    let args = &["--no-color"];
    let expected_output = format!(
        "{a}:1 LeadingCharacter: Invalid leading character detected\n{a}:1 QuoteCharacter: The value is wrapped in quotes\n",
        a=testfile_to_check.shortname_as_str()
    );

    test_dir.test_command_fail_with_args(args, expected_output);
}
