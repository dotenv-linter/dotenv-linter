use crate::common::TestDir;
use std::path::Path;

#[test]
fn checks_one_specific_path() {
    let testdir = TestDir::new();
    testdir.create_testfile(".env", "foo=\n");

    let subdir = testdir.subdir();
    let testfile_2 = subdir.create_testfile(".env.test", "1FOO=\n");

    let args = &[subdir.as_str()];
    let expected_output = format!(
        "{}:1 LeadingCharacter: Invalid leading character detected\n\nFound 1 problem\n",
        Path::new(&testdir.relative_path(&subdir))
            .join(testfile_2.shortname_as_str())
            .to_str()
            .expect("multi-platform path to test .env file")
    );

    testdir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn checks_two_specific_paths() {
    let testdir = TestDir::new();
    testdir.create_testfile(".env", "foo=\n");

    let subdir_1 = testdir.subdir();
    let testfile_2 = subdir_1.create_testfile(".env", " FOO=\n");

    let subdir_2 = subdir_1.subdir();
    let testfile_3 = subdir_2.create_testfile(".env", " FOO=\n");

    let args = &[subdir_1.as_str(), subdir_2.as_str()];
    let expected_output = format!(
        "{}:1 LeadingCharacter: Invalid leading character detected\n{}:1 LeadingCharacter: Invalid leading character detected\n\nFound 2 problems\n",
        Path::new(&testdir.relative_path(&subdir_1))
            .join(testfile_2.shortname_as_str())
            .to_str().expect("multi-platform path to test .env file"),
        Path::new(&testdir.relative_path(&subdir_2))
            .join(testfile_3.shortname_as_str())
            .to_str().expect("multi-platform path to test .env file"),
    );

    testdir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn checks_one_specific_file() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "foo=\n");
    let testfile_2 = test_dir.create_testfile("test-env-file", "FOO =\n");

    let args = &[testfile_2.as_str()];
    let expected_output = format!(
        "{}:1 SpaceCharacter: The line has spaces around equal sign\n\nFound 1 problem\n",
        testfile_2.shortname_as_str()
    );

    test_dir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn checks_two_specific_files() {
    let testdir = TestDir::new();
    testdir.create_testfile(".env", "foo=");
    let testfile_2 = testdir.create_testfile("test-env-file", "FOO =\n");

    let subdir = testdir.subdir();
    let testfile_3 = subdir.create_testfile("another_test_file", "FOO=BAR\nFOO=BAR\n");

    let args = &[testfile_2.as_str(), testfile_3.as_str()];
    let expected_output = format!(
        "{}:2 DuplicatedKey: The FOO key is duplicated\n{}:1 SpaceCharacter: The line has spaces around equal sign\n\nFound 2 problems\n",
        Path::new(&testdir.relative_path(&subdir))
            .join(testfile_3.shortname_as_str())
            .to_str().expect("multi-platform path to test .env file"),
        testfile_2.shortname_as_str(),
    );

    testdir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn checks_each_file_only_once_when_listing_same_path_twice() {
    let testdir = TestDir::new();
    testdir.create_testfile(".env", "foo=\n");

    let subdir = testdir.subdir();
    let testfile_1 = subdir.create_testfile(".env", " FOO=\n");
    let testfile_2 = subdir.create_testfile(".env", " FOO=val\nBAR=foo\n");

    let args = &[subdir.as_str(), subdir.as_str()];
    let expected_output = format!(
        "{}:1 LeadingCharacter: Invalid leading character detected\n{}:2 UnorderedKey: The BAR key should go before the FOO key\n\nFound 2 problems\n",
        Path::new(&testdir.relative_path(&subdir))
            .join(testfile_1.shortname_as_str())
            .to_str().expect("multi-platform path to test .env file"),
        Path::new(&testdir.relative_path(&subdir))
            .join(testfile_2.shortname_as_str())
            .to_str().expect("multi-platform path to test .env file")
    );

    testdir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn checks_each_file_only_once_when_listing_one_path_and_one_file() {
    let testdir = TestDir::new();
    testdir.create_testfile(".env", "foo=\n");

    let subdir = testdir.subdir();
    let testfile_1 = subdir.create_testfile(".env", " FOO=\n");
    let testfile_2 = subdir.create_testfile(".env", " FOO=val\nBAR=foo\n");

    let args = &[subdir.as_str(), testfile_2.as_str()];
    let expected_output = format!(
        "{}:1 LeadingCharacter: Invalid leading character detected\n{}:2 UnorderedKey: The BAR key should go before the FOO key\n\nFound 2 problems\n",
        Path::new(&testdir.relative_path(&subdir))
            .join(testfile_1.shortname_as_str())
            .to_str().expect("multi-platform path to test .env file"),
        Path::new(&testdir.relative_path(&subdir))
            .join(testfile_2.shortname_as_str())
            .to_str().expect("multi-platform path to test .env file")
    );

    testdir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn checks_one_specific_file_and_one_path() {
    let testdir = TestDir::new();
    testdir.create_testfile(".env", "foo=");
    let testfile_2 = testdir.create_testfile("test-env-file", "FOO=BAR\nBAR=FOO\n");

    let subdir = testdir.subdir();
    let testfile_3 = subdir.create_testfile("test.env", "FOO=BAR\nFOO=BAR\n");

    let args = &[testfile_2.as_str(), subdir.as_str()];
    let expected_output = format!(
        "{}:2 DuplicatedKey: The FOO key is duplicated\n{}:2 UnorderedKey: The BAR key should go before the FOO key\n\nFound 2 problems\n",
        Path::new(&testdir.relative_path(&subdir))
            .join(testfile_3.shortname_as_str())
            .to_str().expect("multi-platform path to test .env file"),
        testfile_2.shortname_as_str(),
    );

    testdir.test_command_fail_with_args(args, expected_output);
}

#[test]
fn checks_one_specific_file_twice() {
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", "foo=");
    let testfile_2 = test_dir.create_testfile("test-env-file", "1FOO=\n");

    let args = &[testfile_2.as_str(), testfile_2.as_str()];
    let expected_output = format!(
        "{}:1 LeadingCharacter: Invalid leading character detected\n\nFound 1 problem\n",
        testfile_2.shortname_as_str()
    );

    test_dir.test_command_fail_with_args(args, expected_output);
}
