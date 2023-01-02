use clap::builder::PossibleValue;
use clap::ValueEnum;
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
    ValueWithoutQuotes,
}

impl ValueEnum for LintKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            LintKind::DuplicatedKey,
            LintKind::EndingBlankLine,
            LintKind::ExtraBlankLine,
            LintKind::IncorrectDelimiter,
            LintKind::KeyWithoutValue,
            LintKind::LeadingCharacter,
            LintKind::LowercaseKey,
            LintKind::QuoteCharacter,
            LintKind::SpaceCharacter,
            LintKind::SubstitutionKey,
            LintKind::TrailingWhitespace,
            LintKind::UnorderedKey,
            LintKind::ValueWithoutQuotes,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            LintKind::DuplicatedKey => PossibleValue::new("DuplicatedKey"),
            LintKind::EndingBlankLine => PossibleValue::new("EndingBlankLine"),
            LintKind::ExtraBlankLine => PossibleValue::new("ExtraBlankLine"),
            LintKind::IncorrectDelimiter => PossibleValue::new("IncorrectDelimiter"),
            LintKind::KeyWithoutValue => PossibleValue::new("KeyWithoutValue"),
            LintKind::LeadingCharacter => PossibleValue::new("LeadingCharacter"),
            LintKind::LowercaseKey => PossibleValue::new("LowercaseKey"),
            LintKind::QuoteCharacter => PossibleValue::new("QuoteCharacter"),
            LintKind::SpaceCharacter => PossibleValue::new("SpaceCharacter"),
            LintKind::SubstitutionKey => PossibleValue::new("SubstitutionKey"),
            LintKind::TrailingWhitespace => PossibleValue::new("TrailingWhitespace"),
            LintKind::UnorderedKey => PossibleValue::new("UnorderedKey"),
            LintKind::ValueWithoutQuotes => PossibleValue::new("ValueWithoutQuotes"),
        })
    }
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
            "ValueWithoutQuotes" => Ok(LintKind::ValueWithoutQuotes),
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
        let expected = <LintKind as FromStr>::from_str("DuplicatedKey").unwrap();
        assert_eq!(expected, LintKind::DuplicatedKey);
    }

    #[test]
    fn test_invalid_lint_str_variant() {
        assert_eq!(Err(()), <LintKind as FromStr>::from_str("FooBarLint"));
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
