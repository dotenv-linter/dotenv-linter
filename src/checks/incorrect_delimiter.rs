use crate::checks::Check;
use crate::common::*;

pub(crate) struct IncorrectDelimiterChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl IncorrectDelimiterChecker<'_> {
    fn message(&self, key: &str) -> String {
        self.template.replace("{}", &key)
    }
}

impl Default for IncorrectDelimiterChecker<'_> {
    fn default() -> Self {
        Self {
            name: "IncorrectDelimiter",
            template: "The {} key has incorrect delimiter",
        }
    }
}

impl Check for IncorrectDelimiterChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let key = line.get_key()?;

        // delimiters occur /between/ characters, not as the initial character, so we should
        // remove all invalid leading characters before checking for incorrect delimiters
        let cleaned_key = remove_invalid_leading_chars(&key);

        if cleaned_key
            .trim()
            .chars()
            .any(|c| !c.is_alphanumeric() && c != '_')
        {
            return Some(Warning::new(line.clone(), self.name(), self.message(&key)));
        }

        None
    }

    fn name(&self) -> &str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{check_tester, common::tests::*};

    check_tester!{
        IncorrectDelimiterChecker;
        working_run => {
            "FOO_BAR=FOOBAR" => None,
        },
        working_with_digits_run => {
            "F100=BAR" => None,
        },
        working_with_export_run => {
            "export FOO=BAR" => None,
        },
        incorrect_leading_char => {
            // expect None because this warning should be found by LeadingCharacterChecker
            "*FOO=BAR" => None,
        },
        incorrect_leading_chars_and_invalid_delimiter => {
            "***F-OOBAR=BAZ" => Some("The ***F-OOBAR key has incorrect delimiter"),
        },
        incorrect_ending_delimiter => {
            "FOO*=BAR" => Some("The FOO* key has incorrect delimiter"),
        },
        failing_run => {
            "FOO-BAR=FOOBAR" => Some("The FOO-BAR key has incorrect delimiter"),
        },
        failing_with_whitespace_run => {
            "FOO BAR=FOOBAR" => Some("The FOO BAR key has incorrect delimiter"),
        },
        unformatted_run => {
            "FOO-BAR" => Some("The FOO-BAR key has incorrect delimiter"),
        },
        trailing_space_run => {
            // has a trailing space, so SpaceCharacterChecker should catch this error
            "FOO_BAR =FOOBAR" => None,
        },
        empty_run => {
            "" => None,
        },
        short_run => {
            "F=BAR" => None,
        }
    }

}
