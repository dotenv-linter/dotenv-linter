use std::str::FromStr;

use crate::lint_kind::LintKind;

const PREFIX: &str = "dotenv-linter";
const ON: &str = "on";
const OFF: &str = "off";

#[derive(Debug, PartialEq, Eq)]
pub struct Comment {
    disable: bool,
    pub checks: Vec<LintKind>,
}

impl Comment {
    pub fn parse(s: &str) -> Option<Self> {
        // A comment without the # character
        let line_comment = s.trim_start()[1..].trim();

        // Getting the right side of the comment ["off", "UnorderedKey,", "DuplicatedKey"]
        let flag_with_checks: Vec<&str> = line_comment
            .strip_prefix(format!("{PREFIX}:").as_str())?
            .split_whitespace()
            .collect();

        // Getting a flag and list of checks - ("off", ["UnorderedKey,", "DuplicatedKey"])
        let (&flag, checks) = flag_with_checks.split_first()?;

        if flag != ON && flag != OFF {
            return None;
        }

        // Normalize list of checks. For example:
        // ["UnorderedKey,", "DuplicatedKey"] => ["UnorderedKey", "DuplicatedKey"]
        // ["ExtraBlankLine,LowercaseKey"] => ["ExtraBlankLine", "LowercaseKey"]
        let checks = checks.iter().flat_map(|&s| {
            s.split(',')
                .filter(|&s| !s.is_empty())
                .collect::<Vec<&str>>()
        });

        let disable = flag == OFF;

        // Converting vec of &str lints to `Vec<LintKind>`
        let checks: Vec<LintKind> = checks
            .into_iter()
            .filter_map(|c| LintKind::from_str(c).ok())
            .collect::<Vec<LintKind>>();

        Some(Self { disable, checks })
    }

    pub fn is_disabled(&self) -> bool {
        self.disable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incorrect_comment() {
        assert_eq!(None, Comment::parse("# Simple comment"))
    }

    #[test]
    fn incorrect_comment_without_whitespace() {
        assert_eq!(None, Comment::parse("#Another comment"))
    }

    #[test]
    fn only_prefix() {
        assert_eq!(None, Comment::parse("# dotenv-linter"));
        assert_eq!(None, Comment::parse("# dotenv-linter:"))
    }

    #[test]
    fn invalid_flag() {
        assert_eq!(None, Comment::parse("# dotenv-linter:enable UnorderedKey"));
        assert_eq!(None, Comment::parse("# dotenv-linter:disable UnorderedKey"))
    }

    #[test]
    fn without_flag() {
        assert_eq!(None, Comment::parse("# dotenv-linter UnorderedKey"));
        assert_eq!(None, Comment::parse("# dotenv-linter: UnorderedKey"))
    }

    #[test]
    fn without_checks() {
        assert_eq!(
            Some(Comment {
                disable: false,
                checks: Vec::new(),
            }),
            Comment::parse("# dotenv-linter:on")
        );

        assert_eq!(
            Some(Comment {
                disable: true,
                checks: Vec::new(),
            }),
            Comment::parse("# dotenv-linter:off")
        )
    }

    #[test]
    fn without_whitespace_and_checks() {
        assert_eq!(
            Some(Comment {
                disable: false,
                checks: Vec::new(),
            }),
            Comment::parse("#dotenv-linter:on")
        );

        assert_eq!(
            Some(Comment {
                disable: true,
                checks: Vec::new(),
            }),
            Comment::parse("#dotenv-linter:off")
        )
    }

    #[test]
    fn with_one_check() {
        assert_eq!(
            Some(Comment {
                disable: true,
                checks: vec![LintKind::UnorderedKey],
            }),
            Comment::parse("# dotenv-linter:off UnorderedKey")
        );

        assert_eq!(
            Some(Comment {
                disable: false,
                checks: vec![LintKind::UnorderedKey],
            }),
            Comment::parse("# dotenv-linter:on UnorderedKey")
        );
    }

    #[test]
    fn with_leading_whitespace() {
        assert_eq!(
            Some(Comment {
                disable: true,
                checks: vec![LintKind::UnorderedKey],
            }),
            Comment::parse(" # dotenv-linter:off UnorderedKey")
        );

        assert_eq!(
            Some(Comment {
                disable: false,
                checks: vec![LintKind::UnorderedKey],
            }),
            Comment::parse("  #dotenv-linter:on UnorderedKey")
        );
    }

    #[test]
    fn with_two_checks() {
        assert_eq!(
            Some(Comment {
                disable: true,
                checks: vec![LintKind::UnorderedKey, LintKind::DuplicatedKey],
            }),
            Comment::parse("# dotenv-linter:off UnorderedKey,DuplicatedKey")
        );

        assert_eq!(
            Some(Comment {
                disable: false,
                checks: vec![LintKind::UnorderedKey, LintKind::DuplicatedKey],
            }),
            Comment::parse("# dotenv-linter:on UnorderedKey, DuplicatedKey")
        );

        assert_eq!(
            Some(Comment {
                disable: true,
                checks: vec![LintKind::UnorderedKey, LintKind::DuplicatedKey],
            }),
            Comment::parse("# dotenv-linter:off UnorderedKey ,DuplicatedKey")
        );

        assert_eq!(
            Some(Comment {
                disable: false,
                checks: vec![LintKind::UnorderedKey, LintKind::DuplicatedKey],
            }),
            Comment::parse("# dotenv-linter:on UnorderedKey , DuplicatedKey")
        );

        assert_eq!(
            Some(Comment {
                disable: true,
                checks: vec![LintKind::UnorderedKey, LintKind::DuplicatedKey],
            }),
            Comment::parse("# dotenv-linter:off UnorderedKey,DuplicatedKey,")
        );

        assert_eq!(
            Some(Comment {
                disable: false,
                checks: vec![LintKind::UnorderedKey, LintKind::DuplicatedKey],
            }),
            Comment::parse("# dotenv-linter:on ,UnorderedKey,DuplicatedKey,")
        );
    }

    #[test]
    fn with_some_checks() {
        assert_eq!(
            Some(Comment {
                disable: true,
                checks: vec![
                    LintKind::UnorderedKey,
                    LintKind::DuplicatedKey,
                    LintKind::EndingBlankLine
                ],
            }),
            Comment::parse("# dotenv-linter:off UnorderedKey,DuplicatedKey, EndingBlankLine")
        );

        assert_eq!(
            Some(Comment {
                disable: false,
                checks: vec![
                    LintKind::UnorderedKey,
                    LintKind::DuplicatedKey,
                    LintKind::EndingBlankLine
                ],
            }),
            Comment::parse("# dotenv-linter:on UnorderedKey,DuplicatedKey,   EndingBlankLine")
        );

        assert_eq!(
            Some(Comment {
                disable: true,
                checks: vec![
                    LintKind::UnorderedKey,
                    LintKind::DuplicatedKey,
                    LintKind::EndingBlankLine
                ],
            }),
            Comment::parse(
                "# dotenv-linter:off  ,  UnorderedKey,DuplicatedKey,  EndingBlankLine,   "
            )
        );
    }
}
