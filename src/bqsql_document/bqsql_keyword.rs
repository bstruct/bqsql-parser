#[derive(Debug)]
pub(crate) enum BqsqlKeyword {
    As,
    With,
    Select,
    From,
    Where,
}
impl BqsqlKeyword {
    fn as_str(&self) -> &'static str {
        match self {
            BqsqlKeyword::As => "AS",
            BqsqlKeyword::With => "WITH",
            BqsqlKeyword::Select => "SELECT",
            BqsqlKeyword::From => "FROM",
            BqsqlKeyword::Where => "WHERE",
        }
    }
}
impl PartialEq<&str> for BqsqlKeyword {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == other.to_uppercase()
    }
}

impl PartialEq<BqsqlKeyword> for &str {
    fn eq(&self, other: &BqsqlKeyword) -> bool {
        self.to_uppercase() == other.as_str()
    }
}

#[test]
fn compare_with() {
    assert_eq!(BqsqlKeyword::With, "WITH");
    assert_eq!(BqsqlKeyword::With, "WiTh");
    assert_eq!(BqsqlKeyword::With, "with");
    assert_eq!("WITH", BqsqlKeyword::With);
    assert_eq!("WiTh", BqsqlKeyword::With);
    assert_eq!("with", BqsqlKeyword::With);
}

#[test]
fn compare_select() {
    assert_eq!(BqsqlKeyword::Select, "SELECT");
    assert_eq!(BqsqlKeyword::Select, "SeLeCt");
    assert_eq!(BqsqlKeyword::Select, "select");
    assert_eq!("SELECT", BqsqlKeyword::Select);
    assert_eq!("SeLeCt", BqsqlKeyword::Select);
    assert_eq!("select", BqsqlKeyword::Select);
}

#[test]
fn compare_all() {
    assert_eq!(BqsqlKeyword::As, "AS");
    assert_eq!(BqsqlKeyword::With, "WITH");
    assert_eq!(BqsqlKeyword::Select, "SELECT");
    assert_eq!(BqsqlKeyword::From, "FROM");
    assert_eq!(BqsqlKeyword::Where, "WHERE");
    assert_eq!("AS", BqsqlKeyword::As);
    assert_eq!("WITH", BqsqlKeyword::With);
    assert_eq!("SELECT", BqsqlKeyword::Select);
    assert_eq!("FROM", BqsqlKeyword::From);
    assert_eq!("WHERE", BqsqlKeyword::Where);
}
