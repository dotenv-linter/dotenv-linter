use crate::common::*;

#[test]
fn invisible_character_in_env() {
    let contents = vec!["FOO=test\u{00AD}thing\n"];

    for contents in contents {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", contents);
        let args = &["check", testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[".env:1 InvisibleCharacter: Invisible character U+00AD detected"],
        )]);

        testdir.test_command_fail_with_args(with_default_args(args), expected_output);
    }
}
