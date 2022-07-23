use serde::Serialize;

pub mod parser;
pub mod token_parser;
pub mod parser_test_select;

#[derive(Serialize)]
pub struct BqsqlDocument {
    pub items: Vec<BqsqlDocumentItem>,
}

#[derive(Serialize)]
pub struct BqsqlDocumentItem {
    pub item_type: BqsqlDocumentItemType,
    pub range: Option<[usize;3]>,
    pub items: Vec<BqsqlDocumentItem>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum BqsqlDocumentItemType {
    // UNEXPECTED = -1,
    // UNKNOWN = 0,

    KEYWORD,
    STRING,
    NUMBER,
    OPERATOR,
    // TYPE,
    // FUNCTION,
    // METHOD,

    PARENTHESES_OPEN,
    PARENTHESES_CLOSE,

    AS_ALIAS,
    ALIAS,

    QUERY,

    QUERY_WITH,

    QUERY_SELECT,
    QUERY_SELECT_ALL,
    QUERY_SELECT_DISTINCT,
    QUERY_SELECT_AS_STRUCT,
    QUERY_SELECT_AS_VALUE,
    // QUERY_SELECT_SELECT_LIST,
    QUERY_SELECT_LIST_ITEM,

    // QUERY_FROM,

}