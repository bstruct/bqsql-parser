use crate::bqsql_document::*;
use lazy_static::lazy_static;
use regex::Regex;

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
            column: end,
            line: position.line,
            index: position.index + end,
        }
    }

    pub(crate) fn next(
        text: &str,
        position: &BqsqlDocumentPosition,
    ) -> Option<BqsqlDocumentPosition> {
        if let Some(next_character) = text.chars().nth(position.index) {
            if next_character == '\n' {
                return Some(BqsqlDocumentPosition {
                    column: 0,
                    line: position.line + 1,
                    index: position.index + 1,
                });
            } else {
                return Some(BqsqlDocumentPosition {
                    column: position.column + 1,
                    line: position.line,
                    index: position.index + 1,
                });
            }
        }

        None
    }
}
