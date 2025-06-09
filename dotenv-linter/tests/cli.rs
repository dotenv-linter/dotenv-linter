use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn stdin_flag_works() {
    let mut cmd = Command::cargo_bin("dotenv-linter").unwrap();
    cmd.arg("--stdin")
        .arg("--stdin-filename")
        .arg("CUSTOM.env")
        .write_stdin("KEY=no_space\n")
        .assert()
        .success()
        .stdout(contains("CUSTOM.env"));
}
