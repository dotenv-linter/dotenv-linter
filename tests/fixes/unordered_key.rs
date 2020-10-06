use crate::common::*;

#[test]
fn unordered_key() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(
        ".env",
        "# comment\n\
        \n\
        C=B\n\
        # A comment\n\
        A=B\n\
        X=Y\n\
        # more comments\n\
        \n\
        # middle comment\n\
        \n\
        M=123\n\
        # K comment\n\
        K=123\n\
        # I comment\n\
        # I comment\n\
        I=123\n\
        \n\
        A1=1\n\
        B1=1\n\
        \n\
        # end comment\n",
    );
    let expected_output = fix_output(&[(
        ".env",
        &[
            ".env:5 UnorderedKey: The A key should go before the C key",
            ".env:13 UnorderedKey: The K key should go before the M key",
            ".env:16 UnorderedKey: The I key should go before the K key",
        ],
    )]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "# comment\n\
        \n\
        # A comment\n\
        A=B\n\
        C=B\n\
        X=Y\n\
        # more comments\n\
        \n\
        # middle comment\n\
        \n\
        # I comment\n\
        # I comment\n\
        I=123\n\
        # K comment\n\
        K=123\n\
        M=123\n\
        \n\
        A1=1\n\
        B1=1\n\
        \n\
        # end comment\n",
    );

    testdir.close();
}
