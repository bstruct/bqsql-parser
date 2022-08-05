#[derive(Debug, Clone, Copy)]
pub(crate) enum BqsqlDelimiter {
    ParenthesesOpen,
    ParenthesesClose,
    Dot,
    Comma,
    Semicolon,
}
impl BqsqlDelimiter {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            BqsqlDelimiter::ParenthesesOpen => "(",
            BqsqlDelimiter::ParenthesesClose => ")",
            BqsqlDelimiter::Dot => ".",
            BqsqlDelimiter::Comma => ",",
            BqsqlDelimiter::Semicolon => ";",
        }
    }
}
impl PartialEq<&str> for BqsqlDelimiter {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == other.to_string()
    }
}

impl PartialEq<BqsqlDelimiter> for &str {
    fn eq(&self, other: &BqsqlDelimiter) -> bool {
        self.to_string() == other.as_str()
    }
}

#[test]
fn compare_all() {
    assert_eq!(BqsqlDelimiter::ParenthesesOpen, "(");
    assert_eq!(BqsqlDelimiter::ParenthesesClose, ")");
    assert_eq!(BqsqlDelimiter::Dot, ".");
    assert_eq!(BqsqlDelimiter::Comma, ",");
    assert_eq!(BqsqlDelimiter::Semicolon, ";");
    assert_eq!("(", BqsqlDelimiter::ParenthesesOpen);
    assert_eq!(")", BqsqlDelimiter::ParenthesesClose);
    assert_eq!(".", BqsqlDelimiter::Dot);
    assert_eq!(",", BqsqlDelimiter::Comma);
    assert_eq!(";", BqsqlDelimiter::Semicolon);
}
