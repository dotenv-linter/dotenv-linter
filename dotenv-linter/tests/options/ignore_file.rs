use crate::common::*;

// Auto-detect .envignore in project root
#[test]
fn autodetect_envignore_excludes_files() {
    let testdir = TestDir::new();

    // Create .env files
    let _a = testdir.create_testfile(".env", "FOO=bar baz\n"); // would warn
    let _b = testdir.create_testfile(".env.local", "BAR=ok\n"); // should be ignored

    // Create .envignore
    let _ignore = testdir.create_testfile(".envignore", ".env.local\n");

    // Expect only .env is reported, .env.local is ignored
    let args: [&str; 0] = [];
    let expected_output = check_output(&[(
        ".env",
        &[".env:1 ValueWithoutQuotes: This value needs to be surrounded in quotes"],
    )]);

    testdir.test_command_fail_with_args(with_default_args(&args), expected_output);
}

// Use --ignore-file with a custom ignore list
#[test]
fn custom_ignore_file_flag_excludes_files() {
    let testdir = TestDir::new();

    // Create .env files
    let _a = testdir.create_testfile("app.env", "FOO=bar baz\n"); // would warn
    let _b = testdir.create_testfile("secrets.env", "BAR=ok\n"); // should be ignored via custom file

    // Create custom ignore file
    let ignore = testdir.create_testfile("ignore.list", "secrets.env\n");

    // Expect only app.env is reported, secrets.env is ignored
    let args = ["--ignore-file", ignore.as_str()];
    let expected_output = check_output(&[(
        "app.env",
        &["app.env:1 ValueWithoutQuotes: This value needs to be surrounded in quotes"],
    )]);

    testdir.test_command_fail_with_args(with_default_args(&args), expected_output);
}

// Patterns support
#[test]
fn ignore_patterns_are_supported() {
    let testdir = TestDir::new();

    // Create .env files that would warn
    let _a = testdir.create_testfile(".env.dev", "FOO=bar baz\n");
    let _b = testdir.create_testfile(".env.prod", "FOO=bar baz\n");

    // Create .envignore pattern
    let _ignore = testdir.create_testfile(".envignore", ".env.*\n");

    // Expect none reported because both match pattern
    let args: [&str; 0] = [];
    let expected_output = "Nothing to check\n";

    testdir.test_command_success_with_args(with_default_args(&args), expected_output);
}
