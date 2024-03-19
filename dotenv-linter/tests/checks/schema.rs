use crate::common::*;
use std::{
    fs::{remove_file, File},
    io::Write,
};
use tempfile::tempdir;

#[test]
fn load_good_schema() {
    let json = r#"{
        "version": "1.0.0",
        "entries": [
            {
                "key": "NAME",
                "type": "String"
            },
            {
                "key": "PORT",
                "type": "Integer"
            },
            {
                "key": "PRICE",
                "type": "Float"
            },
            {
                "key": "URL",
                "type": "Url"
            },
            {
                "key": "EMAIL",
                "type": "Email"
            },
            {
                "key": "FLAG",
                "type": "Boolean"
            }
        ]
    }"#;
    // write the above json to a temp file
    let temp_dir = tempdir().expect("create temp dir");
    let file_path = temp_dir.path().join("schema.json");
    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
    let content = "NAME=JOE\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &["-S", file_path.to_str().unwrap(), testfile.as_str()];

    let expected_output = check_output(&[(".env", &[])]);

    testdir.test_command_success_with_args(with_default_args(args), expected_output);
    let _ = remove_file(file_path);
}

#[test]
fn load_bad_schema() {
    let json = r#"{
        "version": "1.0.0",
        bad_json
    }"#;
    // write the above json to a temp file

    let testdir = TestDir::new();
    let test_schema = testdir.create_testfile("schema.json", json);

    let content = "NAME=JOE\n";
    let testfile = testdir.create_testfile(".env", content);
    let args = &["-S", test_schema.as_str(), testfile.as_str()];

    let expected_output = "Error loading schema: key must be a string at line 3 column 9\n";

    testdir.test_command_fail_with_args(with_default_args(args), expected_output);
}

#[test]
fn load_missing_schema() {
    let content = "NAME=JOE\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &["-S", "no_such_file", testfile.as_str()];
    let expected_output = if cfg!(target_os = "windows") {
        "Error loading schema: The system cannot find the file specified. (os error 2)\n"
    } else {
        "Error loading schema: No such file or directory (os error 2)\n"
    };
    testdir.test_command_fail_with_args(with_default_args(args), expected_output);
}
