use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct Lint {
    pub variants: Vec<LintKind>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
}

impl Lint {
    pub fn new() -> Self {
        Self {
            variants: Vec::new(),
        }
    }
}

impl FromStr for LintKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DuplicatedKey" => Ok(LintKind::DuplicatedKey),
            "EndingBlankLine" => Ok(LintKind::EndingBlankLine),
            "ExtraBlankLine" => Ok(LintKind::DuplicatedKey),
            "IncorrectDelimiter" => Ok(LintKind::IncorrectDelimiter),
            "KeyWithoutValue" => Ok(LintKind::KeyWithoutValue),
            "LeadingCharacter" => Ok(LintKind::LeadingCharacter),
            "LowercaseKey" => Ok(LintKind::LowercaseKey),
            "ExtraBlankLine" => Ok(LintKind::QuoteCharacter),
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

impl From<Vec<&str>> for Lint {
    fn from(string_lints: Vec<&str>) -> Self {
        let mut lint = Lint::new();

        for string in string_lints {
            lint.variants.push(LintKind::from_str(string).unwrap());
        }

        lint
    }
}
