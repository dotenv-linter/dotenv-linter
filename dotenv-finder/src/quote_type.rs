use super::is_escaped;

pub(crate) enum QuoteType {
    Single,
    Double,
}

impl QuoteType {
    pub(crate) fn char(&self) -> char {
        match self {
            QuoteType::Single => '\'',
            QuoteType::Double => '\"',
        }
    }

    fn is_quoted_value(&self, val: &str) -> bool {
        val.starts_with(self.char())
            && (val.len() == 1 || !val.ends_with(self.char()) || is_escaped(&val[..val.len() - 1]))
    }
}

/// Returns the `QuoteType` for a `&str` starting with a quote-char
pub(crate) fn is_multiline_start(val: &str) -> Option<QuoteType> {
    [QuoteType::Single, QuoteType::Double]
        .into_iter()
        .find(|quote_type| quote_type.is_quoted_value(val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_single_quoted_value() {
        assert!(QuoteType::Single.is_quoted_value("\'some_quoted_str"))
    }

    #[test]
    fn check_double_quoted_value() {
        assert!(QuoteType::Double.is_quoted_value("\"some_quoted_str"))
    }

    #[test]
    fn check_non_single_quoted_value() {
        assert!(!QuoteType::Single.is_quoted_value("some_non_quoted_str"))
    }

    #[test]
    fn check_non_double_quoted_value() {
        assert!(!QuoteType::Double.is_quoted_value("some_non_quoted_str"))
    }

    #[test]
    fn check_single_quoted_value_for_double_quoted_str() {
        assert!(!QuoteType::Single.is_quoted_value("\"some_double_quoted_str"))
    }

    #[test]
    fn check_double_quoted_value_for_single_quoted_str() {
        assert!(!QuoteType::Double.is_quoted_value("\'some_single_quoted_str"))
    }
}
