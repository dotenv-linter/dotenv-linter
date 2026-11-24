use dotenv_core::LineEntry;

use super::Check;
use crate::{LintKind, Warning};

pub(crate) struct ValueWithoutQuotesChecker<'a> {
    template: &'a str,
}

impl ValueWithoutQuotesChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }

    /// Returns the value without an inline comment.
    ///
    /// A `#` starts a comment only when it is outside quotes, is not escaped,
    /// and follows a whitespace. Values like `foo#bar` and `"foo # bar"` are
    /// kept as is.
    fn value_without_comment(value: &str) -> &str {
        let mut quote = None;
        let mut escaped = false;
        let mut is_after_whitespace = false;
        let mut is_start = true;

        for (index, ch) in value.char_indices() {
            let was_escaped = escaped;
            escaped = false;

            if !was_escaped {
                if ch == '\\' {
                    escaped = true;
                } else if let Some(quote_char) = quote {
                    if ch == quote_char {
                        quote = None;
                    }
                } else if is_start && (ch == '\'' || ch == '"') {
                    quote = Some(ch);
                } else if ch == '#' && is_after_whitespace {
                    return value[..index].trim_end();
                }
            }

            is_after_whitespace = !was_escaped && ch.is_whitespace();
            if !ch.is_whitespace() {
                is_start = false;
            }
        }

        value
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
        let val = Self::value_without_comment(line.get_value()?).trim();

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
