use crate::common::*;

#[test]
fn stdin_with_valid_content() {
    let testdir = TestDir::new();

    let mut cmd = assert_cmd::Command::cargo_bin("dotenv-linter").expect("binary");
    cmd.current_dir(testdir.as_str())
        .args(with_default_args(&["check", "--stdin"]))
        .write_stdin("FOO=bar\nBAR=baz\n")
        .assert()
        .success();
}

#[test]
fn stdin_with_invalid_content() {
    let testdir = TestDir::new();

    let expected_output = check_output(&[(
        "stdin",
        &["stdin:1 SpaceCharacter: The line has spaces around equal sign"],
    )]);

    let mut cmd = assert_cmd::Command::cargo_bin("dotenv-linter").expect("binary");
    cmd.current_dir(testdir.as_str())
        .args(with_default_args(&["check", "--stdin"]))
        .write_stdin("FOO =bar\n")
        .assert()
        .failure()
        .code(1)
        .stdout(expected_output);
}

#[test]
fn stdin_with_empty_content() {
    let testdir = TestDir::new();

    let mut cmd = assert_cmd::Command::cargo_bin("dotenv-linter").expect("binary");
    cmd.current_dir(testdir.as_str())
        .args(with_default_args(&["check", "--stdin"]))
        .write_stdin("")
        .assert()
        .success();
}

#[test]
fn stdin_with_multiple_warnings() {
    let testdir = TestDir::new();

    let expected_output = check_output(&[(
        "stdin",
        &[
            "stdin:1 LowercaseKey: The foo key should be in uppercase",
            "stdin:2 SpaceCharacter: The line has spaces around equal sign",
        ],
    )]);

    let mut cmd = assert_cmd::Command::cargo_bin("dotenv-linter").expect("binary");
    cmd.current_dir(testdir.as_str())
        .args(with_default_args(&["check", "--stdin"]))
        .write_stdin("foo=bar\nBAR =baz\n")
        .assert()
        .failure()
        .code(1)
        .stdout(expected_output);
}
