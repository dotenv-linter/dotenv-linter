use crate::common::TestDir;

#[test]
fn leading_character() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "*BAR=BAZ\n.ABC=DEF\n1QUX=QUUX\n_FOO=BAR\n");
    let expected_output = String::from(
        "Fixed warnings:\n\
        .env:1 LeadingCharacter: Invalid leading character detected\n\
        .env:2 LeadingCharacter: Invalid leading character detected\n\
        .env:3 LeadingCharacter: Invalid leading character detected\n",
    );

    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "ABC=DEF\nBAR=BAZ\nQUX=QUUX\n_FOO=BAR\n"
    );

    testdir.close();
}
