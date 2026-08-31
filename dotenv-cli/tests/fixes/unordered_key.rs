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

#[test]
fn unordered_key_with_comment_delimited_groups() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(
        ".env",
        "ENV2=bbb\nENV1=aaa\n\n###> group1 ###\nENV4=ddd\nENV5=eee\nENV3=ccc\n###< group1 ###\n\nENV7=ggg\nENV6=fff\n",
    );
    let expected_output = fix_output(&[(
        ".env",
        &[
            ".env:2 UnorderedKey: The ENV1 key should go before the ENV2 key",
            ".env:7 UnorderedKey: The ENV3 key should go before the ENV4 key",
            ".env:11 UnorderedKey: The ENV6 key should go before the ENV7 key",
        ],
    )]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "ENV1=aaa\nENV2=bbb\n\n###> group1 ###\nENV3=ccc\nENV4=ddd\nENV5=eee\n###< group1 ###\n\nENV6=fff\nENV7=ggg\n",
    );

    testdir.close();
}
