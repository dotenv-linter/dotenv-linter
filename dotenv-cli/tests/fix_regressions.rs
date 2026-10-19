use std::process::Command;

fn run_fix(
    contents: &str,
    path: &str,
) -> (tempfile::TempDir, std::path::PathBuf, std::process::Output) {
    let dir = tempfile::tempdir().unwrap();
    let env_file = dir.path().join(path);
    std::fs::write(&env_file, contents).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_dotenv-linter"))
        .current_dir(dir.path())
        .args(["fix", path, "--no-backup", "--plain"])
        .output()
        .unwrap();
    (dir, env_file, output)
}

#[test]
fn cli_fix_preserves_supported_export_assignment() {
    let (_dir, env_file, output) = run_fix("export foo=bar\n", ".env");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(env_file).unwrap(),
        "export FOO=bar\n"
    );
}

#[test]
fn cli_fix_handles_spaces_before_first_equals_with_equals_in_quoted_value() {
    let (_dir, env_file, output) = run_fix("AUTH_TOKEN = \"abc==\"\n", ".env");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(env_file).unwrap(),
        "AUTH_TOKEN=\"abc==\"\n"
    );
}

#[test]
fn cli_fix_missing_explicit_path_does_not_rewrite_cwd_dotenv_file() {
    let dir = tempfile::tempdir().unwrap();
    let sentinel = dir.path().join(".env");
    std::fs::write(&sentinel, "foo=bar\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_dotenv-linter"))
        .current_dir(dir.path())
        .args(["fix", "typo.env", "--no-backup", "--plain"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(std::fs::read(sentinel).unwrap(), b"foo=bar\n");
}
