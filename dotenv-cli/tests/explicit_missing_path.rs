use dotenv_linter::{FixOptions, fix};

#[test]
fn missing_explicit_fix_path_does_not_rewrite_current_directory() {
    let dir = tempfile::tempdir().unwrap();
    let env_file = dir.path().join(".env");
    std::fs::write(&env_file, "foo=bar\n").unwrap();
    let missing = dir.path().join("typo.env");
    let opts = FixOptions {
        files: vec![&missing],
        ignore_checks: vec![],
        exclude: vec![],
        quiet: true,
        recursive: false,
        no_backup: true,
        dry_run: false,
    };
    assert!(fix(&opts, &dir.path().to_path_buf()).is_err());
    assert_eq!(std::fs::read_to_string(env_file).unwrap(), "foo=bar\n");
}
