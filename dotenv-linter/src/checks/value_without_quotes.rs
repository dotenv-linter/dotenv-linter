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
                ("FOO=BAR # Some Comment", None),
                ("FOO=BAR BAZ # Some Comment", Some(WARNING)),
            ],
        );
    }
}
