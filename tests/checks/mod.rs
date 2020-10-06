use crate::common::*;

mod ending_blank_line;
mod extra_blank_line;

#[test]
fn no_problems() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "A1=1\n");
    let args = &[testfile.as_str()];

    let expected_output = check_output(&[(".env", &[])]);

    testdir.test_command_success_with_args(args, expected_output);
}
