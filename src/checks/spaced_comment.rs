use crate::checks::Check;
use crate::common::*;

pub(crate) struct SpacedCommentChecker<'a> {
    name: &'a str,
    template: &'a str,
}

impl Default for SpacedCommentChecker<'_> {
    fn default() -> Self {
        Self {
            name: "SpacedComment",
            template: "Expected space after '#' in comment",
        }
    }
}

impl SpacedCommentChecker<'_> {
    fn message(&self) -> String {
        String::from(self.template)
    }
}

impl Check for SpacedCommentChecker<'_> {
    fn run(&mut self, line: &LineEntry) -> Option<Warning> {
        if line.is_comment() && !line.raw_string.starts_with("# ") {
            return Some(Warning::new(line.clone(), self.name(), self.message()));
        }

        None
    }

    fn name(&self) -> &str {
        self.name
    }

    fn skip_comments(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn with_space() {
        let mut checker = SpacedCommentChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },

            raw_string: String::from("# This is a good comment\n"),
        };

        assert_eq!(None, checker.run(&line));
    }

    #[test]
    fn without_space() {
        let mut checker = SpacedCommentChecker::default();
        let line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("#This is a bad comment\n"),
        };
        let expected = Some(Warning::new(
            line.clone(),
            "SpacedComment",
            String::from("Expected space after '#' in comment"),
        ));

        assert_eq!(expected, checker.run(&line));
    }
}
