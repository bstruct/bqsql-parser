use serde::Serialize;

#[derive(Serialize)]
pub struct BqsqlDocument {
    pub document_type: BqsqlDocumentType,
    pub items: Vec<BqsqlDocumentItem>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum BqsqlDocumentType {
    UNKNOWN = 0,
    QUERY = 1,
}

#[derive(Serialize)]
pub struct BqsqlDocumentItem {
    pub item_type: BqsqlDocumentItemType,
    pub from: BqsqlDocumentPosition,
    pub to: BqsqlDocumentPosition,
}

#[derive(Serialize)]
pub struct BqsqlDocumentPosition {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum BqsqlDocumentItemType {
    UNKNOWN = 0,
    COMMENT = 1,
    QUERY = 2,
}

impl Default for BqsqlDocumentItemType {
    fn default() -> Self {
        Self::UNKNOWN
    }
}
