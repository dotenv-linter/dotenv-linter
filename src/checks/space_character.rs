use crate::checks::Check;
use crate::common::*;

pub(crate) struct SpaceCharacterChecker<'a> {
    template: &'a str,
    name: &'a str,
}

impl SpaceCharacterChecker<'_> {
    fn message(&self) -> &str {
        self.template
    }
}

impl Default for SpaceCharacterChecker<'_> {
    fn default() -> Self {
        Self {
            name: "SpaceCharacter",
            template: "The line has spaces around equal sign",
        }
    }
}

impl Check for SpaceCharacterChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let line_splitted = line.raw_string.split('=').collect::<Vec<&str>>();

        if let [key, value] = &line_splitted[..] {
            if key.ends_with(' ') || value.starts_with(' ') {
                return Some(Warning::new(line.clone(), self.name(), self.message()));
            }
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

    const MESSAGE: &str = "The line has spaces around equal sign";

    check_tester!{
        SpaceCharacterChecker;
        working_run => {
            "DEBUG_HTTP=true" => None,
        },
        working_leading_run => {
            " DEBUG_HTTP=true" => None,
        },
        working_trailing_run => {
            "DEBUG_HTTP=true " => None,
        },
        working_empty_run => {
            "" => None,
        },
        working_no_equal_sign_run => {
            "DEBUG_HTTP true" => None,
        },
        failing_run => {
            "DEBUG-HTTP = true" => Some(MESSAGE),
        },
        failing_when_whitespace_before_equal_sign_run => {
            "DEBUG-HTTP =true" => Some(MESSAGE),
        },
        failing_when_whitespace_after_equal_sign_run => {
            "DEBUG-HTTP= true" => Some(MESSAGE),
        }
    }
}
