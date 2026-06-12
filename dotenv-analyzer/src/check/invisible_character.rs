use dotenv_core::LineEntry;

use super::Check;
use crate::{LintKind, Warning};

pub(crate) struct InvisibleCharacter<'a> {
    template: &'a str,
}

impl InvisibleCharacter<'_> {
    fn message(&self, c: char) -> String {
        self.template.replace("{}", &format!("U+{:04X}", c as u32))
    }
}

impl Default for InvisibleCharacter<'_> {
    fn default() -> Self {
        Self {
            template: "Invisible character {} detected",
        }
    }
}

const NON_ASCII_INVISIBLE: [char; 7] = [
    '\u{00AD}', //&shy;
    '\u{200B}', //&ZeroWidthSpace;
    '\u{200C}', //&zwnj;
    '\u{200D}', //&zwj;
    '\u{200E}', //&lrm;
    '\u{200F}', //&rlm;
    '\u{FEFF}', //&zwnbsp;
];

impl Check for InvisibleCharacter<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        let chars = line.raw_string.chars();

        for c in chars {
            if (c.is_whitespace() && !c.is_ascii()) || NON_ASCII_INVISIBLE.contains(&c) {
                return Some(Warning::new(line.number, self.name(), self.message(c)));
            }
        }

        None
    }

    fn name(&self) -> LintKind {
        LintKind::InvisibleCharacter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::check_test;

    #[test]
    fn working_run_with_value() {
        check_test(&mut InvisibleCharacter::default(), [("FOO=BAR", None)]);
    }

    #[test]
    fn accented_e() {
        check_test(&mut InvisibleCharacter::default(), [("é", None)]);
    }

    #[test]
    fn micro() {
        check_test(&mut InvisibleCharacter::default(), [("µ", None)]);
    }

    #[test]
    fn ae() {
        check_test(&mut InvisibleCharacter::default(), [("Æ", None)]);
    }

    #[test]
    fn e() {
        check_test(&mut InvisibleCharacter::default(), [("ë", None)]);
    }

    #[test]
    fn space() {
        check_test(&mut InvisibleCharacter::default(), [(" ", None)]);
    }

    #[test]
    fn hello_world() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("FOO=hello world!", None)],
        );
    }

    //nbsp
    #[test]
    fn non_breaking_space() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{00A0}", Some("Invisible character U+00A0 detected"))],
        );
    }

    //&shy;
    #[test]
    fn soft_hyphen() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{00AD}", Some("Invisible character U+00AD detected"))],
        );
    }

    // &ensp;
    #[test]
    fn en_space() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{2002}", Some("Invisible character U+2002 detected"))],
        );
    }

    // &emsp;
    #[test]
    fn em_space() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{2003}", Some("Invisible character U+2003 detected"))],
        );
    }

    // &thinsp;
    #[test]
    fn thin_space() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{2009}", Some("Invisible character U+2009 detected"))],
        );
    }

    // &ZeroWidthSpace;
    #[test]
    fn zero_width_space() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{200B}", Some("Invisible character U+200B detected"))],
        );
    }

    // &zwnj;;
    #[test]
    fn zero_width_non_joiner() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{200C}", Some("Invisible character U+200C detected"))],
        );
    }

    // &zwj;
    #[test]
    fn zero_width_joiner() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{200D}", Some("Invisible character U+200D detected"))],
        );
    }

    // &lrm;;
    #[test]
    fn left_to_right_mark() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{200E}", Some("Invisible character U+200E detected"))],
        );
    }

    // &rlm;
    #[test]
    fn right_to_left_mark() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{200F}", Some("Invisible character U+200F detected"))],
        );
    }

    // &zwnbsp;
    #[test]
    fn zero_width_no_bsp() {
        check_test(
            &mut InvisibleCharacter::default(),
            [("\u{FEFF}", Some("Invisible character U+FEFF detected"))],
        );
    }

    #[test]
    fn non_breaking_space_end_of_text() {
        check_test(
            &mut InvisibleCharacter::default(),
            [(
                "QWERTY=\u{00A0}",
                Some("Invisible character U+00A0 detected"),
            )],
        );
    }

    #[test]
    fn non_breaking_space_in_text() {
        check_test(
            &mut InvisibleCharacter::default(),
            [(
                "QWERTY=ba\u{00A0}r",
                Some("Invisible character U+00A0 detected"),
            )],
        );
    }
}
