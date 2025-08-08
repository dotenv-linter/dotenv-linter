use crate::common::*;

#[test]
fn unordered_key() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(
        ".env",
        "# comment\n\nC=B\n# A comment\nA=B\nX=Y\n# more comments\n\n# middle comment\n\nM=123\n# \
         K comment\nK=123\n# I comment\n# I comment\nI=123\n\nA1=1\nB1=1\n\n# end comment\n",
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
        "# comment\n\n# A comment\nA=B\nC=B\nX=Y\n# more comments\n\n# middle comment\n\n# I \
         comment\n# I comment\nI=123\n# K comment\nK=123\nM=123\n\nA1=1\nB1=1\n\n# end comment\n",
    );

    testdir.close();
}
