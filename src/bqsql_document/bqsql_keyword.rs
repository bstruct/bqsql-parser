#[derive(Debug)]
pub(crate) enum BqsqlKeyword {
    All,
    As,
    Distinct,
    From,
    Recursive,
    Select,
    Struct,
    Value,
    Where,
    With,
}
impl BqsqlKeyword {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            BqsqlKeyword::As => "AS",
            BqsqlKeyword::All => "ALL",
            BqsqlKeyword::Distinct => "DISTINCT",
            BqsqlKeyword::From => "FROM",
            BqsqlKeyword::Recursive => "RECURSIVE",
            BqsqlKeyword::Select => "SELECT",
            BqsqlKeyword::Struct => "STRUCT",
            BqsqlKeyword::Value => "VALUE",
            BqsqlKeyword::Where => "WHERE",
            BqsqlKeyword::With => "WITH",
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
    assert_eq!(BqsqlKeyword::Recursive, "recursive");
    assert_eq!(BqsqlKeyword::Select, "SELECT");
    assert_eq!(BqsqlKeyword::From, "FROM");
    assert_eq!(BqsqlKeyword::Where, "WHERE");
    assert_eq!("AS", BqsqlKeyword::As);
    assert_eq!("WITH", BqsqlKeyword::With);
    assert_eq!("recursive", BqsqlKeyword::Recursive);
    assert_eq!("SELECT", BqsqlKeyword::Select);
    assert_eq!("FROM", BqsqlKeyword::From);
    assert_eq!("WHERE", BqsqlKeyword::Where);
}
