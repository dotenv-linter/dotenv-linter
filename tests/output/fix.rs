//! Tests that output from fixes are correct. Mainly needed to ensure that
//! newlines are printed correctly.
use crate::common::*;
use std::fs;

#[test]
fn warnings() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "abc=DEF\n");

    let args: &[&str] = &[];
    let expected_output = String::from(
        r#"Fixing .env
.env:1 LowercaseKey: The abc key should be in uppercase

All warnings are fixed. Total: 1
"#,
    );

    test_dir.test_command_fix_success_with_args(expected_output, args);
    test_dir.close();
}

#[test]
fn warnings_multiple_files() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "abc=DEF\n");
    test_dir.create_testfile(".env_1", "ABC=DEF\n\n");
    test_dir.create_testfile(".env_2", "ABC=DEF\nABC=DEF\n");

    let args: &[&str] = &[];
    let expected_output = String::from(
        r#"Fixing .env
.env:1 LowercaseKey: The abc key should be in uppercase

Fixing .env_1
.env_1:3 ExtraBlankLine: Extra blank line detected

Fixing .env_2
.env_2:2 DuplicatedKey: The ABC key is duplicated

All warnings are fixed. Total: 3
"#,
    );

    test_dir.test_command_fix_success_with_args(expected_output, args);
    test_dir.close();
}

#[test]
fn no_warnings() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "ABC=DEF\nB=bbb\nF=BAR\n");

    let args: &[&str] = &[];
    let expected_output = String::from(
        r#"Fixing .env

No warnings found
"#,
    );

    test_dir.test_command_fix_success_with_args(expected_output, args);
    test_dir.close();
}

#[test]
fn no_warnings_multiple_files() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "ABC=DEF\nB=bbb\nF=BAR\n");
    test_dir.create_testfile(".env_1", "ABC=DEF\nB=bbb\nF=BAR\n");
    test_dir.create_testfile(".env_2", "ABC=DEF\nB=bbb\nF=BAR\n");

    let args: &[&str] = &[];
    let expected_output = String::from(
        r#"Fixing .env
Fixing .env_1
Fixing .env_2

No warnings found
"#,
    );

    test_dir.test_command_fix_success_with_args(expected_output, args);
    test_dir.close();
}

#[test]
fn mixed_warnings_multiple_files() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "abc=DEF\n");
    test_dir.create_testfile(".env_1", "ABC=DEF\n");
    test_dir.create_testfile(".env_2", "ABC=DEF\nABC=DEF\n");

    let args: &[&str] = &[];
    let expected_output = String::from(
        r#"Fixing .env
.env:1 LowercaseKey: The abc key should be in uppercase

Fixing .env_1
Fixing .env_2
.env_2:2 DuplicatedKey: The ABC key is duplicated

All warnings are fixed. Total: 2
"#,
    );

    test_dir.test_command_fix_success_with_args(expected_output, args);

    test_dir.create_testfile(".env_3", "ABC=DEF\n\n");
    let expected_output = String::from(
        r#"Fixing .env
Fixing .env_1
Fixing .env_2
Fixing .env_3
.env_3:3 ExtraBlankLine: Extra blank line detected

All warnings are fixed. Total: 1
"#,
    );

    test_dir.test_command_fix_success_with_args(expected_output, args);

    test_dir.create_testfile(".env", "ABC=DEF\n\n");
    let expected_output = String::from(
        r#"Fixing .env
.env:3 ExtraBlankLine: Extra blank line detected

Fixing .env_1
Fixing .env_2
Fixing .env_3

All warnings are fixed. Total: 1
"#,
    );

    test_dir.test_command_fix_success_with_args(expected_output, args);

    test_dir.close();
}

#[test]
fn quiet() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "abc=DEF\n\nF=BAR\nB=bbb\n");

    let args = &["--quiet"];
    let expected_output = String::from(
        r#"
All warnings are fixed. Total: 2
"#,
    );

    test_dir.test_command_fix_success_with_args(expected_output, args);
    test_dir.close();
}

#[test]
fn quiet_no_warnings() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "ABC=DEF\nB=bbb\nF=BAR\n");

    let args = &["--quiet"];
    let expected_output = String::from(
        r#"
No warnings found
"#,
    );

    test_dir.test_command_fix_success_with_args(expected_output, args);
    test_dir.close();
}

#[test]
fn backup() {
    let test_dir = TestDir::new();
    let test_file = test_dir.create_testfile(".env", "abc=DEF\n\nF=BAR\nB=bbb\n");

    let args = &["-f"];
    let output = test_dir.test_command_success_and_get_output(args);

    let backup_file = fs::read_dir(&test_dir.as_str())
        .expect("read dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().as_os_str() != test_file.as_str())
        .last()
        .expect("get backup file");
    let backup_filename = backup_file
        .file_name()
        .into_string()
        .expect("convert to string");
    let expected_output = format!(
        r#"Fixing .env
Original file was backed up to: "{}"

.env:1 LowercaseKey: The abc key should be in uppercase
.env:4 UnorderedKey: The B key should go before the F key

All warnings are fixed. Total: 2
"#,
        backup_filename
    );
    assert_eq!(output, expected_output);

    test_dir.close();
}

#[test]
fn quiet_backup() {
    let test_dir = TestDir::new();
    let test_file = test_dir.create_testfile(".env", "abc=DEF\n\nF=BAR\nB=bbb\n");

    let args = &["-f", "-q"];
    let output = test_dir.test_command_success_and_get_output(args);

    let backup_file = fs::read_dir(&test_dir.as_str())
        .expect("read dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().as_os_str() != test_file.as_str())
        .last()
        .expect("get backup file");
    let backup_filename = backup_file
        .file_name()
        .into_string()
        .expect("convert to string");
    let expected_output = format!(
        r#"Original file was backed up to: "{}"

All warnings are fixed. Total: 2
"#,
        backup_filename
    );
    assert_eq!(output, expected_output);

    test_dir.close();
}
