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

impl Check for ValueWithoutQuotesChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let val = line.get_value()?.trim();

        // Helper function to check if we're inside quotes at a given position
        fn is_in_quotes(s: &str, pos: usize) -> bool {
            let mut in_single = false;
            let mut in_double = false;

            for (i, c) in s.chars().enumerate() {
                if i >= pos {
                    break;
                }
                match c {
                    '\'' if !in_double => in_single = !in_single,
                    '"' if !in_single => in_double = !in_double,
                    _ => {}
                }
            }
            in_single || in_double
        }

        let comment_start = val
            .chars()
            .enumerate()
            .find(|&(i, c)| c == '#' && !is_in_quotes(val, i))
            .map(|(i, _)| i);

        let content = match comment_start {
            Some(pos) => val[..pos].trim(),
            None => val,
        };

        if content.contains(char::is_whitespace)
            && !(content.starts_with('\'') && content.ends_with('\''))
            && !(content.starts_with('\"') && content.ends_with('\"'))
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
                ("FOO=BAR # Some Comment", None),
                ("FOO=BAR BAZ # Some Comment", Some(WARNING)),
                ("FOO=\"Pas#word\"", None),
                ("FOO=\"Pas #word\"", None),
                ("FOO=\"Pas#word\" # Some Comment", None),
                ("FOO=\"Pas #word\" # Some Comment", None),
                ("FOO=\"Pas#word\"  Some Comment", Some(WARNING)),
                ("FOO=\"Pas #word\"  Some Comment", Some(WARNING)),
            ],
        );
    }
}
