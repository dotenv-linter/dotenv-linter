//! Tests that output from fixes are correct. Mainly needed to ensure that
//! newlines are printed correctly.
use crate::common::*;

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
