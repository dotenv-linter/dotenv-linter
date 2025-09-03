use super::is_escaped;

pub(crate) enum Quote {
    Single,
    Double,
}

impl Quote {
    pub(crate) fn as_char(&self) -> char {
        match self {
            Quote::Single => '\'',
            Quote::Double => '\"',
        }
    }

    fn is_quoted(&self, val: &str) -> bool {
        val.starts_with(self.as_char())
            && (val.len() == 1
                || !val.ends_with(self.as_char())
                || is_escaped(&val[..val.len() - 1]))
    }
}

/// Returns the `Quote` for a `&str` starting with a quote-char
pub(crate) fn get_quote(val: &str) -> Option<Quote> {
    [Quote::Single, Quote::Double]
        .into_iter()
        .find(|q| q.is_quoted(val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_quoted() {
        assert!(Quote::Single.is_quoted("\'some_quoted_str"))
    }

    #[test]
    fn test_double_quoted() {
        assert!(Quote::Double.is_quoted("\"some_quoted_str"))
    }

    #[test]
    fn test_non_single_quoted() {
        assert!(!Quote::Single.is_quoted("some_non_quoted_str"))
    }

    #[test]
    fn test_non_double_quoted() {
        assert!(!Quote::Double.is_quoted("some_non_quoted_str"))
    }

    #[test]
    fn test_single_quoted_for_double_quoted_str() {
        assert!(!Quote::Single.is_quoted("\"some_double_quoted_str"))
    }

    #[test]
    fn test_double_quoted_for_single_quoted_str() {
        assert!(!Quote::Double.is_quoted("\'some_single_quoted_str"))
    }
}
