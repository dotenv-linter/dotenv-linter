use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LintKind {
    DuplicatedKey,
    EndingBlankLine,
    ExtraBlankLine,
    IncorrectDelimiter,
    KeyWithoutValue,
    LeadingCharacter,
    LowercaseKey,
    QuoteCharacter,
    SpaceCharacter,
    SubstitutionKey,
    TrailingWhitespace,
    UnorderedKey,
    Unfixable,
}

impl FromStr for LintKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DuplicatedKey" => Ok(LintKind::DuplicatedKey),
            "EndingBlankLine" => Ok(LintKind::EndingBlankLine),
            "ExtraBlankLine" => Ok(LintKind::ExtraBlankLine),
            "IncorrectDelimiter" => Ok(LintKind::IncorrectDelimiter),
            "KeyWithoutValue" => Ok(LintKind::KeyWithoutValue),
            "LeadingCharacter" => Ok(LintKind::LeadingCharacter),
            "LowercaseKey" => Ok(LintKind::LowercaseKey),
            "QuoteCharacter" => Ok(LintKind::QuoteCharacter),
            "SpaceCharacter" => Ok(LintKind::SpaceCharacter),
            "SubstitutionKey" => Ok(LintKind::SubstitutionKey),
            "TrailingWhitespace" => Ok(LintKind::TrailingWhitespace),
            "UnorderedKey" => Ok(LintKind::UnorderedKey),
            _ => Err(()),
        }
    }
}

impl fmt::Display for LintKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).replace("LintKind", ""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_lint_variant_conversion() {
        let lint_as_str = "DuplicatedKey";
        let expected = LintKind::from_str(lint_as_str).unwrap();

        assert_eq!(expected, LintKind::DuplicatedKey);
    }

    #[test]
    fn test_invalid_lint_str_variant() {
        let invalid_lint_str = "FooBarLint";
        let expected = Err(());

        assert_eq!(expected, LintKind::from_str(invalid_lint_str));
    }

    #[test]
    fn test_lint_variant_display_output() {
        let one = LintKind::EndingBlankLine;
        let two = LintKind::DuplicatedKey;
        let three = LintKind::SubstitutionKey;

        assert_eq!(
            "EndingBlankLine DuplicatedKey SubstitutionKey",
            format!("{} {} {}", one, two, three)
        );
    }
}
