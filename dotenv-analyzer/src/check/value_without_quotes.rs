use dotenv_core::{LineEntry, split_inline_comment};

use super::Check;
use crate::{LintKind, Warning};

pub(crate) struct ValueWithoutQuotesChecker<'a> {
    template: &'a str,
}

impl ValueWithoutQuotesChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for ValueWithoutQuotesChecker<'_> {
    fn default() -> Self {
        Self {
            template: "This value needs to be surrounded in quotes",
        }
    }
}

impl Check for ValueWithoutQuotesChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let val = split_inline_comment(line.get_value()?).0.trim();

        if val.contains(char::is_whitespace)
            && !(val.starts_with('\'') && val.ends_with('\''))
            && !(val.starts_with('\"') && val.ends_with('\"'))
        {
            Some(Warning::new(line.number, self.name(), self.message()))
        } else {
            None
        }
    }

    fn name(&self) -> LintKind {
        LintKind::ValueWithoutQuotes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::check_test;

    const WARNING: &str = "This value needs to be surrounded in quotes";

    #[test]
    fn value_without_quotes() {
        check_test(
            &mut ValueWithoutQuotesChecker::default(),
            [
                ("FOO=BAR", None),
                ("FOO=BAR BAZ", Some(WARNING)),
                ("FOO=\"BAR BAZ\"", None),
                ("FOO=\'BAR BAR\'", None),
            ],
        );
    }

    #[test]
    fn inline_comment() {
        check_test(
            &mut ValueWithoutQuotesChecker::default(),
            [
                ("FOO=bar # comment", None),
                ("FOO=bar   # comment", None),
                ("FOO=bar\t# comment", None),
                ("FOO= # comment", None),
                ("FOO=\"bar baz\" # comment", None),
                ("FOO='bar baz' # comment", None),
                ("FOO=\"bar # baz\"", None),
                ("FOO='bar # baz'", None),
                ("FOO=bar#baz", None),
                ("FOO=it's # comment", None),
                ("FOO=bar baz # comment", Some(WARNING)),
                ("FOO=bar baz#qux", Some(WARNING)),
                ("FOO=\"bar\" baz # comment", Some(WARNING)),
                ("FOO=test \\# comment", Some(WARNING)),
                ("FOO=тест # комментарий", None),
            ],
        );
    }
}
