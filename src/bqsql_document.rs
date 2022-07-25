use serde::Serialize;

pub mod parser;
pub mod parser_test_select;
pub mod token_parser;

#[derive(Serialize, Clone)]
pub struct BqsqlDocument {
    pub items: Vec<BqsqlDocumentItem>,
}

#[derive(Serialize, Clone)]
pub struct BqsqlDocumentItem {
    pub item_type: BqsqlDocumentItemType,
    pub range: Option<[usize; 3]>,
    pub items: Vec<BqsqlDocumentItem>,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub enum BqsqlDocumentItemType {
    // UNEXPECTED = -1,
    // UNKNOWN = 0,
    Keyword,
    String,
    Number,
    Operator,
    // TYPE,
    // FUNCTION,
    // METHOD,
    ParenthesesOpen,
    ParenthesesClose,
    Comma,

    AsAlias,
    Alias,

    Query,

    // QueryWith,

    QuerySelect,
    QuerySelectAll,
    QuerySelectDistinct,
    QuerySelectAsStruct,
    QuerySelectAsValue,
    // QUERY_SELECT_SELECT_LIST,
    QuerySelectListItem,
    // QUERY_FROM,
}
