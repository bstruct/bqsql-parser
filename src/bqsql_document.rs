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
    UNKNOWN = 0,
    QUERY = 1,
}

#[derive(Serialize)]
pub struct BqsqlDocumentToken {
    pub token: String,
    pub from: BqsqlDocumentPosition,
    pub to: BqsqlDocumentPosition,
}

#[derive(Serialize)]
pub struct BqsqlDocumentPosition {
    pub line: usize,
    pub character: usize,
}

impl BqsqlDocumentPosition {
    pub(crate) fn new(line: usize, character: usize) -> BqsqlDocumentPosition {
        BqsqlDocumentPosition {
            line: line,
            character: character,
        }
    }
}