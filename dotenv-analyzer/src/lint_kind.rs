use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LintKind {
    DuplicatedKey,
    EndingBlankLine,
    ExtraBlankLine,
    IncorrectDelimiter,
    InvisibleCharacter,
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

impl LintKind {
    pub const ALL: [LintKind; 15] = [
        LintKind::DuplicatedKey,
        LintKind::EndingBlankLine,
        LintKind::ExtraBlankLine,
        LintKind::IncorrectDelimiter,
        LintKind::InvisibleCharacter,
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
    ];

    pub const fn name(self) -> &'static str {
        match self {
            LintKind::DuplicatedKey => "DuplicatedKey",
            LintKind::EndingBlankLine => "EndingBlankLine",
            LintKind::ExtraBlankLine => "ExtraBlankLine",
            LintKind::IncorrectDelimiter => "IncorrectDelimiter",
            LintKind::InvisibleCharacter => "InvisibleCharacter",
            LintKind::KeyWithoutValue => "KeyWithoutValue",
            LintKind::LeadingCharacter => "LeadingCharacter",
            LintKind::LowercaseKey => "LowercaseKey",
            LintKind::QuoteCharacter => "QuoteCharacter",
            LintKind::SpaceCharacter => "SpaceCharacter",
            LintKind::SubstitutionKey => "SubstitutionKey",
            LintKind::TrailingWhitespace => "TrailingWhitespace",
            LintKind::UnorderedKey => "UnorderedKey",
            LintKind::ValueWithoutQuotes => "ValueWithoutQuotes",
            LintKind::SchemaViolation => "SchemaViolation",
        }
    }
}

impl FromStr for LintKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.name() == s)
            .ok_or(())
    }
}

impl fmt::Display for LintKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for LintKind {
    fn value_variants<'a>() -> &'a [Self] {
        &Self::ALL
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.name()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_lint_variant_conversion() {
        let expected = <LintKind as FromStr>::from_str("DuplicatedKey").expect("parse lint");
        assert_eq!(expected, LintKind::DuplicatedKey);
    }

    #[test]
    fn test_invalid_lint_str_variant() {
        assert_eq!(Err(()), <LintKind as FromStr>::from_str("FooBarLint"));
    }

    #[test]
    fn test_all_lint_kinds_roundtrip() {
        for kind in LintKind::ALL {
            assert_eq!(Ok(kind), LintKind::from_str(kind.name()), "{kind:?}");
        }
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
