use crate::common::TestDir;
use std::fs;

#[test]
fn output_backup_file() {
    // test that a backup file has been created
    let testdir = TestDir::new();
    let content = "foo=bar\n";
    let testfile = testdir.create_testfile(".env", content);
    let args = &[testfile.as_str(), "fix"];

    testdir.test_command_success_with_args_without_closing(args);

    let backup_file = fs::read_dir(&testdir.as_str())
        .expect("read dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().as_os_str() != testfile.as_str())
        .last()
        .expect("get backup file");

    let backup_contents = fs::read_to_string(backup_file.path()).expect("read backup file");

    assert_eq!(backup_contents, content);
    assert_eq!(testfile.contents(), "FOO=bar\n");
    assert_eq!(backup_file.path().extension().unwrap(), "bak");

    testdir.close()
}
