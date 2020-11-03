use crate::common::{check_output, TestDir};
use std::path::Path;

#[test]
fn checks_one_in_subdir() {
    let test_dir = TestDir::new();
    test_dir.create_testfile("correct.env", "FOO=BAR\n");
    let test_subdir = test_dir.subdir();
    let testfile_2 = test_subdir.create_testfile(".incorrect.env", "1BAR=\n");
    let testfile_2_pathbuf =
        Path::new(&test_dir.relative_path(&test_subdir)).join(testfile_2.shortname_as_str());
    let testfile_2_path = testfile_2_pathbuf
        .to_str()
        .expect("multi-platform path to test .env file");

    let args = &["-r"];
    let expected_output = check_output(&[
        (
            testfile_2_path,
            &[format!(
                "{}:1 LeadingCharacter: Invalid leading character detected",
                testfile_2_path
            )
            .as_str()],
        ),
        ("correct.env", &[]),
    ]);

    test_dir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn checks_files_in_deep_subdirs() {
    let test_dir = TestDir::new();
    test_dir.create_testfile("correct.env", "FOO=BAR\n");

    let test_subdir_2 = test_dir.subdir();
    let testfile_2 = test_subdir_2.create_testfile("incorrect.sub_1.env", "FOO=BAR\nBAR=FOO\n");
    let testfile_2_pathbuf =
        Path::new(&test_dir.relative_path(&test_subdir_2)).join(testfile_2.shortname_as_str());
    let testfile_2_path = testfile_2_pathbuf
        .to_str()
        .expect("multi-platform path to test .env file");

    let test_subdir_3 = test_subdir_2.subdir();
    let testfile_3 = test_subdir_3.create_testfile(".incorrect.env", "FOO=");
    let testfile_3_pathbuf =
        Path::new(&test_dir.relative_path(&test_subdir_3)).join(testfile_3.shortname_as_str());
    let testfile_3_path = testfile_3_pathbuf
        .to_str()
        .expect("multi-platform path to test .env file");

    let args = &["--recursive"];
    let expected_output = check_output(&[
        (
            testfile_3_path,
            &[format!(
                "{}:1 EndingBlankLine: No blank line at the end of the file",
                testfile_3_path
            )
            .as_str()],
        ),
        (
            testfile_2_path,
            &[format!(
                "{}:2 UnorderedKey: The BAR key should go before the FOO key",
                testfile_2_path
            )
            .as_str()],
        ),
        ("correct.env", &[]),
    ]);

    test_dir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn checks_without_recursive_flag() {
    let test_dir = TestDir::new();
    test_dir.create_testfile("correct.env", "FOO=BAR\n");
    let test_subdir = test_dir.subdir();
    test_subdir.create_testfile(".incorrect.env", "1BAR=\n");

    let expected_output = check_output(&[("correct.env", &[])]);

    // incorrect file located in a subdirectory should not be checked
    test_dir.test_command_success(expected_output);
}

#[test]
fn checks_recursive_with_exclude_subdir() {
    let test_dir = TestDir::new();
    test_dir.create_testfile("correct.env", "FOO=BAR\n");

    let test_subdir_2 = test_dir.subdir();
    let testfile_2 = test_subdir_2.create_testfile("incorrect.sub_1.env", "FOO=BAR\nBAR=FOO\n");
    let testfile_2_pathbuf =
        Path::new(&test_dir.relative_path(&test_subdir_2)).join(testfile_2.shortname_as_str());
    let testfile_2_path = testfile_2_pathbuf
        .to_str()
        .expect("multi-platform path to test .env file");

    let test_subdir_3 = test_subdir_2.subdir();
    let testfile_to_exclude = test_subdir_3.create_testfile(".incorrect.env", "FOO=");

    let args = &["--exclude", testfile_to_exclude.as_str(), "--recursive"];
    let expected_output = check_output(&[
        (
            testfile_2_path,
            &[format!(
                "{}:2 UnorderedKey: The BAR key should go before the FOO key",
                testfile_2_path,
            )
            .as_str()],
        ),
        ("correct.env", &[]),
    ]);

    test_dir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn checks_nofollow_subdir_symlinks() {
    let test_dir = TestDir::new();
    let test_subdir = test_dir.subdir();
    let testfile = test_subdir.create_testfile(".incorrect.env", "1BAR=\n");
    let testfile_pathbuf =
        Path::new(&test_dir.relative_path(&test_subdir)).join(testfile.shortname_as_str());
    let testfile_path = testfile_pathbuf
        .to_str()
        .expect("multi-platform path to test .env file");
    // create a symbolic link to its containing directory
    test_subdir.create_symlink(&test_subdir, "symlink");

    let args = &["-r"];
    let expected_output = check_output(&[(
        testfile_path,
        &[format!(
            "{}:1 LeadingCharacter: Invalid leading character detected",
            testfile_path
        )
        .as_str()],
    )]);

    test_dir.test_command_fail_with_args(args, expected_output);
}
