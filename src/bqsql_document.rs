use lazy_static::lazy_static;
use regex::Regex;
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

// #[derive(Serialize)]
// pub struct BqsqlDocumentItemComment {
//     pub from: BqsqlDocumentPosition,
//     pub to: BqsqlDocumentPosition,
// }

#[derive(Serialize)]
pub struct BqsqlDocumentPosition {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

impl BqsqlDocumentPosition {
    pub(crate) fn beginning_text() -> BqsqlDocumentPosition {
        BqsqlDocumentPosition {
            column: 0,
            line: 0,
            index: 0,
        }
    }

    pub(crate) fn copy(position: &BqsqlDocumentPosition) -> BqsqlDocumentPosition {
        BqsqlDocumentPosition {
            column: position.column,
            line: position.line,
            index: position.index,
        }
    }

    pub(crate) fn move_to_non_white(
        text: &str,
        position: &BqsqlDocumentPosition,
    ) -> BqsqlDocumentPosition {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\s*)").unwrap();
        }
        if let Some(white_space_match) = RE.find_at(text, position.index) {
            return BqsqlDocumentPosition::move_end(position, white_space_match.end());
        }

        BqsqlDocumentPosition::copy(position)
    }

    pub(crate) fn move_end(position: &BqsqlDocumentPosition, end: usize) -> BqsqlDocumentPosition {
        BqsqlDocumentPosition {
            column: position.column + end,
            line: position.line,
            index: position.index + end,
        }
    }
    
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum BqsqlDocumentItemType {
    UNKNOWN = 0,
    COMMENT = 1,
    QUERY = 2,
}
