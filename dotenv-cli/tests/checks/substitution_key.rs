use crate::common::*;

#[test]
fn correct_files() {
    let contents = [
        "ABC=$BAR\nBAR=$ABC\n",
        "FOO=${BAR}\n",
        "A=B\nFOO=\"$BAR\"\n",
        "FOO=$ABC{${BAR}\nBIZ=$FOO-$ABC\n",
        "ABC=\\${BAR\n",
    ];

    for content in contents {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &["check", testfile.as_str()];

        let expected_output = check_output(&[(".env", &[])]);

        testdir.test_command_success_with_args(with_default_args(args), expected_output);
    }
}

#[test]
fn incorrect_files() {
    let contents = [
        "ABV=TEST\nFOO=${BAR\n",
        "A=${BAR!}FOO=B4\n",
        "TEST=$BAR}\n",
        "FOO=${ABC-$BAR}\n",
    ];
    let expected = [(2, "FOO"), (1, "A"), (1, "TEST"), (1, "FOO")];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &["check", testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[format!(
                ".env:{} SubstitutionKey: The {} key is not assigned properly",
                expected[i].0, expected[i].1,
            )
            .as_str()],
        )]);

        testdir.test_command_fail_with_args(with_default_args(args), expected_output);
    }
}
