use assert_cmd::Command;

fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("command from binary name")
}

#[test]
fn check_stdin_with_warnings() {
    let expected_output = "Checking .env\n\
         .env:2 LowercaseKey: The foo key should be in uppercase\n\
         \nFound 1 problem\n";

    cmd()
        .args(["check", "--stdin", "--skip-updates"])
        .write_stdin("FOO=bar\nfoo=baz\n")
        .assert()
        .failure()
        .code(1)
        .stdout(expected_output);
}

#[test]
fn check_stdin_without_warnings() {
    let expected_output = "Checking .env\n\nNo problems found\n";

    cmd()
        .args(["check", "--stdin", "--skip-updates"])
        .write_stdin("BAR=baz\nFOO=bar\n")
        .assert()
        .success()
        .stdout(expected_output);
}

#[test]
fn check_stdin_uses_custom_filename() {
    let expected_output = "Checking .env.production\n\
         .env.production:2 LowercaseKey: The foo key should be in uppercase\n\
         \nFound 1 problem\n";

    cmd()
        .args([
            "check",
            "--stdin",
            "--stdin-filename",
            ".env.production",
            "--skip-updates",
        ])
        .write_stdin("FOO=bar\nfoo=baz\n")
        .assert()
        .failure()
        .code(1)
        .stdout(expected_output);
}

#[test]
fn fix_stdin_prints_fixed_content_without_writing_to_disk() {
    let output = cmd()
        .args(["fix", "--stdin"])
        .write_stdin("foo=bar\n")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let output = String::from_utf8(output).expect("utf8 output");

    assert!(output.starts_with("Fixing .env\n"));
    assert!(output.contains("Dry run - not changing any files on disk."));
    assert!(output.contains("FOO=bar"));
    assert!(output.contains(".env:1 LowercaseKey: The foo key should be in uppercase"));
    assert!(output.trim_end().ends_with("All warnings are fixed. Total: 1"));
}

#[test]
fn fix_stdin_without_warnings() {
    let expected_output = "Fixing .env\n\nNo warnings found\n";

    cmd()
        .args(["fix", "--stdin"])
        .write_stdin("FOO=bar\n")
        .assert()
        .success()
        .stdout(expected_output);
}
