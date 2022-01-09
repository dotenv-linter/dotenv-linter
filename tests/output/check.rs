//! Tests that output from checks are correct. Mainly needed to ensure that
//! newlines are printed correctly.
use crate::common::*;

#[test]
fn problems() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "abc=DEF\n");

    let expected_output = r#"Checking .env
.env:1 LowercaseKey: The abc key should be in uppercase

Found 1 problem
"#;

    test_dir.test_command_fail_with_args(with_default_args(&[]), expected_output);
}

#[test]
fn problems_multiple_files() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "abc=DEF\n");
    test_dir.create_testfile(".env_1", "ABC=DEF\n\n");
    test_dir.create_testfile(".env_2", "ABC=DEF\nABC=DEF\n");

    let expected_output = r#"Checking .env
.env:1 LowercaseKey: The abc key should be in uppercase

Checking .env_1
.env_1:3 ExtraBlankLine: Extra blank line detected

Checking .env_2
.env_2:2 DuplicatedKey: The ABC key is duplicated

Found 3 problems
"#;

    test_dir.test_command_fail_with_args(with_default_args(&[]), expected_output);
}

#[test]
fn valid_multiline_value_test() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(
        ".env",
        "FOO=bar\nMULTILINE_1='{\n\"first\": 1,\n\"second\": 1\n}'\nMULTILINE_2='multiline \\'escaped\\' \n value'\nZAC=baz\n",
    );
    test_dir.create_testfile(".env1", "MULTILINE=\"\n{\n'key':'value'\n}\n\"\n");
    test_dir.create_testfile(".env2", "MULTILINE=\"[\nkey=value\n]\"\n");

    let expected_output = r#"Checking .env
Checking .env1
Checking .env2

No problems found
"#;

    test_dir.test_command_success_with_args(with_default_args(&[]), expected_output);
}

#[test]
fn valid_double_quoted_multiline_value_test() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(
        ".env",
        "FOO=bar\nMULTILINE_1=\"#yml example \n first: 1 \n second: 1 \n\"\nMULTILINE_2=\"multiline \\\"escaped\\\" \n value\"\nZAC=baz\n",
    );

    let expected_output = r#"Checking .env

No problems found
"#;

    test_dir.test_command_success_with_args(with_default_args(&[]), expected_output);
}

#[test]
fn problems_first_and_last_file() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "abc=DEF\n");
    test_dir.create_testfile(".env_1", "ABC=DEF\n");
    test_dir.create_testfile(".env_2", "ABC=DEF\nABC=DEF\n");

    let expected_output = r#"Checking .env
.env:1 LowercaseKey: The abc key should be in uppercase

Checking .env_1
Checking .env_2
.env_2:2 DuplicatedKey: The ABC key is duplicated

Found 2 problems
"#;

    test_dir.test_command_fail_with_args(with_default_args(&[]), expected_output);
}

#[test]
fn problems_middle_file() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "ABC=DEF\n");
    test_dir.create_testfile(".env_1", "ABC=DEF\n\n");
    test_dir.create_testfile(".env_2", "ABC=DEF\n");

    let expected_output = r#"Checking .env
Checking .env_1
.env_1:3 ExtraBlankLine: Extra blank line detected

Checking .env_2

Found 1 problem
"#;

    test_dir.test_command_fail_with_args(with_default_args(&[]), expected_output);
}

#[test]
fn no_problems() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "ABC=DEF\nB=bbb\nF=BAR\n");

    let expected_output = r#"Checking .env

No problems found
"#;

    test_dir.test_command_success_with_args(with_default_args(&[]), expected_output);
}

#[test]
fn no_problems_multiple_files() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "ABC=DEF\nB=bbb\nF=BAR\n");
    test_dir.create_testfile(".env_1", "ABC=DEF\nB=bbb\nF=BAR\n");
    test_dir.create_testfile(".env_2", "ABC=DEF\nB=bbb\nF=BAR\n");

    let expected_output = r#"Checking .env
Checking .env_1
Checking .env_2

No problems found
"#;

    test_dir.test_command_success_with_args(with_default_args(&[]), expected_output);
}

#[test]
fn no_files() {
    let test_dir = TestDir::new();

    let expected_output = r#"Nothing to check
"#;

    test_dir.test_command_success_with_args(with_default_args(&[]), expected_output);
}

#[test]
fn quiet() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "abc=DEF\n\nF=BAR\nB=bbb\n");

    let args = &["--quiet"];
    let expected_output = r#".env:1 LowercaseKey: The abc key should be in uppercase
.env:4 UnorderedKey: The B key should go before the F key
"#;

    test_dir.test_command_fail_with_args(with_default_args(args), expected_output);
}

#[test]
fn quiet_no_problems() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "ABC=DEF\nB=bbb\nF=BAR\n");

    let args = &["--quiet"];
    let expected_output = "";

    test_dir.test_command_success_with_args(with_default_args(args), expected_output);
}

#[test]
fn quiet_no_files() {
    let test_dir = TestDir::new();

    let args = &["--quiet"];
    let expected_output = "";

    test_dir.test_command_success_with_args(with_default_args(args), expected_output);
}
