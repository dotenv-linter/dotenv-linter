#[allow(dead_code)]
mod cli_common;

use cli_common::TestDir;

fn test_autofix(given: &str, fixed: &str) {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", given);

    testdir.test_command_fix_file(&testfile, fixed);
}

#[test]
fn correct_file() {
    test_autofix(
        "ABC=DEF\nD=BAR\n\nFOO=BAR\n",
        "ABC=DEF\nD=BAR\n\nFOO=BAR\n"
    );
}

#[test]
fn extra_blank_lines() {
    test_autofix(
        "\n\nABC=DEF\nD=BAR\n\n\nFOO=BAR\n\n",
        "\nABC=DEF\nD=BAR\n\nFOO=BAR\n"
    );
}

#[test]
fn ending_blank_line() {
    test_autofix(
        "ABC=DEF\nD=BAR\n\nFOO=BAR",
        "ABC=DEF\nD=BAR\n\nFOO=BAR\n"
    );
}

#[test]
fn lowercase_key() {
    test_autofix(
        "abc=DEF\nD=BAR\n\nfOO=BAR\n",
        "ABC=DEF\nD=BAR\n\nFOO=BAR\n"
    );
}

#[test]
fn unordered_keys() {
    test_autofix(
        "FOO=BAR\nD=BAR\nABC=DEF\n",
        "ABC=DEF\nD=BAR\nFOO=BAR\n"
    );

    test_autofix(
        "FOO=BAR\nD=BAR\nABC=DEF\n# comment\n",
        "ABC=DEF\nD=BAR\nFOO=BAR\n# comment\n"
    );

    test_autofix(
        "FOO=BAR\nD=BAR\n#comment\nABC=DEF\n",
        "#comment\nABC=DEF\nD=BAR\nFOO=BAR\n"
    );

    test_autofix(
        "FOO=BAR\nD=BAR\n\n#comment\n\nABC=DEF\n",
        "\n#comment\n\nABC=DEF\nD=BAR\nFOO=BAR\n"
    );
}

#[test]
fn different_fixes() {
    test_autofix(
        "#cmt\nfOO=BAR\nD=BAR\n\n\n\n\n\nABC=DEF",
        "\nABC=DEF\nD=BAR\n#cmt\nFOO=BAR\n"
    );
}
