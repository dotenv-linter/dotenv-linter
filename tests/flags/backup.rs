use crate::common::TestDir;
use std::fs;

#[test]
fn output_backup_file() {
    // test that a backup file has been created
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "foo=bar\n");
    let args = &[testfile.as_str(), "-f"];

    testdir.test_command_success_with_args_without_closing(args);

    // test .env was corrected
    assert_eq!(testfile.contents(), "FOO=bar\n");

    // test backup filewas created and has original contents

    // remove the .env file from the paths in current directory so we can examine backup
    let backup_path = fs::read_dir(&testdir.as_str())
        .unwrap()
        .filter_map(|dir_entry| dir_entry.ok())
        .skip_while(|dir_entry| {
            if let Some(current_path) = dir_entry.path().to_str() {
                return current_path == testfile.as_str();
            }
            false
        })
        .collect::<Vec<fs::DirEntry>>();
    let backup_contents =
        String::from_utf8_lossy(&fs::read(backup_path[0].path()).expect("read file")).into_owned();
    assert_eq!(backup_contents, "foo=bar\n");

    testdir.close()
}
