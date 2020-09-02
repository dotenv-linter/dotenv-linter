use crate::common::TestDir;

#[test]
fn output_backup_file() {
    // test that a backup file has been created
    let test_dir = TestDir::new();
    test_dir.create_testfile(".env", " BAR='Baz'\n");

    test_dir.test_command_fix_backs_up_files();
}
