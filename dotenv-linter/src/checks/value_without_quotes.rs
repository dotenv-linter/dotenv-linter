use super::Check;
use crate::common::{LintKind, Warning};
use dotenv_lookup::LineEntry;

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

// this will handle inline comments
#[allow(dead_code)]
struct ValueWithComment {
    value: String,
    comment: String,
}

impl ValueWithComment {
    fn is_quoted(&self) -> bool {
        let value = self.value.trim();
        (value.starts_with('\'') && value.ends_with('\''))
            || (value.starts_with('\"') && value.starts_with('\"'))
    }
}

impl ValueWithoutQuotesChecker<'_> {
    fn split_value_and_comment(&self, value: &str) -> Option<ValueWithComment> {
        let mut in_quotes = false;
        let mut quote_char = None;

        for (i, c) in value.chars().enumerate() {
            match c {
                '\'' | '\"' if !in_quotes => {
                    in_quotes = true;
                    quote_char = Some(c);
                }
                c if Some(c) == quote_char => {
                    in_quotes = false;
                    quote_char = None;
                }
                '#' if !in_quotes => {
                    let val = value[..i].trim();
                    let comment = value[i..].trim();

                    return Some(ValueWithComment {
                        value: val.to_string(),
                        comment: comment.to_string(),
                    });
                }
                _ => {}
            }
        }
        None
    }
}
impl Check for ValueWithoutQuotesChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let val = line.get_value()?.trim();

        if let Some(value_with_comment) = self.split_value_and_comment(val) {
            if value_with_comment.value.contains(char::is_whitespace)
                && !value_with_comment.is_quoted()
            {
                return Some(Warning::new(line.number, self.name(), self.message()));
            } else {
                return None;
            }
        }
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
    use crate::common::tests::check_test;

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
    fn with_regular_comments() {
        check_test(
            &mut ValueWithoutQuotesChecker::default(),
            [
                ("FOO=test # comment", None),
                ("FOO=test value # comment", Some(WARNING)),
                ("FOO=\"test value\" # comment", None),
                ("FOO='test value' # comment", None),
            ],
        );
    }

    #[test]
    fn with_quoted_hashes() {
        check_test(
            &mut ValueWithoutQuotesChecker::default(),
            [
                ("FOO=\"test # not a comment\"", None),
                ("FOO='test # not a comment'", None),
            ],
        );
    }
}
