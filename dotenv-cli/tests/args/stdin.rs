use crate::common::*;

#[test]
fn check_reads_from_stdin() {
    let test_dir = TestDir::new();
    let expected_output = r#"Checking -
-:1 LowercaseKey: The abc key should be in uppercase

Found 1 problem
"#;

    test_dir
        .test_command_with_stdin(with_default_args(&["check", "-"]), "abc=DEF\n")
        .failure()
        .code(1)
        .stdout(expected_output)
        .stderr("");

    test_dir.close();
}

#[test]
fn check_does_not_scan_files_when_reading_stdin() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "zzz=DEF\n");
    let expected_output = r#"Checking -

No problems found
"#;

    test_dir
        .test_command_with_stdin(with_default_args(&["check", "-"]), "ABC=DEF\n")
        .success()
        .stdout(expected_output)
        .stderr("");

    test_dir.close();
}

#[test]
fn check_stdin_quiet() {
    let test_dir = TestDir::new();
    let expected_output = "-:1 LowercaseKey: The abc key should be in uppercase\n";

    test_dir
        .test_command_with_stdin(with_default_args(&["check", "-", "--quiet"]), "abc=DEF\n")
        .failure()
        .code(1)
        .stdout(expected_output)
        .stderr("");

    test_dir.close();
}

#[test]
fn stdin_cannot_be_combined_with_files() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "ABC=DEF\n");

    let assert =
        test_dir.test_command_with_stdin(with_default_args(&["check", "-", ".env"]), "ABC=DEF\n");

    let assert = assert.failure().code(2);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(stderr.contains("standard input ('-') cannot be combined with other paths"));

    test_dir.close();
}

#[test]
fn diff_does_not_support_stdin() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "ABC=DEF\n");

    let assert = test_dir.test_command_with_stdin(["diff", "-", ".env"], "");

    let assert = assert.failure().code(2);
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(stderr.contains("the diff command does not support standard input ('-')"));

    test_dir.close();
}
