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
    SchemaViolation,
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
        write!(f, "{}", format!("{self:?}").replace("LintKind", ""))
    }
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for LintKind {
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
            LintKind::SchemaViolation,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            LintKind::DuplicatedKey => clap::builder::PossibleValue::new("DuplicatedKey"),
            LintKind::EndingBlankLine => clap::builder::PossibleValue::new("EndingBlankLine"),
            LintKind::ExtraBlankLine => clap::builder::PossibleValue::new("ExtraBlankLine"),
            LintKind::IncorrectDelimiter => clap::builder::PossibleValue::new("IncorrectDelimiter"),
            LintKind::KeyWithoutValue => clap::builder::PossibleValue::new("KeyWithoutValue"),
            LintKind::LeadingCharacter => clap::builder::PossibleValue::new("LeadingCharacter"),
            LintKind::LowercaseKey => clap::builder::PossibleValue::new("LowercaseKey"),
            LintKind::QuoteCharacter => clap::builder::PossibleValue::new("QuoteCharacter"),
            LintKind::SpaceCharacter => clap::builder::PossibleValue::new("SpaceCharacter"),
            LintKind::SubstitutionKey => clap::builder::PossibleValue::new("SubstitutionKey"),
            LintKind::TrailingWhitespace => clap::builder::PossibleValue::new("TrailingWhitespace"),
            LintKind::UnorderedKey => clap::builder::PossibleValue::new("UnorderedKey"),
            LintKind::ValueWithoutQuotes => clap::builder::PossibleValue::new("ValueWithoutQuotes"),
            LintKind::SchemaViolation => clap::builder::PossibleValue::new("SchemaViolation"),
        })
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
            format!("{one} {two} {three}")
        );
    }
}
