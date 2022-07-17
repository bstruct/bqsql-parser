use crate::bqsql_document::*;
use lazy_static::lazy_static;
use regex::Regex;

impl BqsqlDocument {
    pub(crate) fn parse(bqsql: &str) -> BqsqlDocument {
        let mut document_type = BqsqlDocumentType::UNKNOWN;
        let mut position = BqsqlDocumentPosition::beginning_text();
        let mut items = Vec::new();

        while position.index < bqsql.len() {
            let mut skip = false;

            if let Some(comment) = handle_comment(bqsql, &position) {
                position = BqsqlDocumentPosition::copy(&comment.to);
                items.push(comment);
                skip = true;
            }

            if !skip {
                if let Some(query) = handle_query(bqsql, &position) {
                    document_type = BqsqlDocumentType::QUERY;
                    position = BqsqlDocumentPosition::copy(&query.to);
                    items.push(query);
                }
            }

            if let Some(next_position) = BqsqlDocumentPosition::next(bqsql, &position) {
                position = next_position;
            } else {
                break;
            }
        }

        BqsqlDocument {
            document_type: document_type,
            items: items,
        }
    }
}

fn handle_comment(bqsql: &str, position: &BqsqlDocumentPosition) -> Option<BqsqlDocumentItem> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*--.*").unwrap();
    }
    if let Some(comment_match) = RE.find_at(bqsql, position.index) {
        let end = comment_match.end();

        return Some(BqsqlDocumentItem {
            item_type: BqsqlDocumentItemType::COMMENT,
            from: BqsqlDocumentPosition::move_to_non_white(bqsql, position),
            to: BqsqlDocumentPosition::move_end(position, end - 1),
        });
    }
    None
}

fn handle_query(bqsql: &str, position: &BqsqlDocumentPosition) -> Option<BqsqlDocumentItem> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[\t\v\f\r ]*SELECT\s+").unwrap();
    }

    if let Some(select_match) = RE.find(&bqsql[position.index..]) {
        let end = select_match.end();

        return Some(BqsqlDocumentItem {
            item_type: BqsqlDocumentItemType::QUERY,
            from: BqsqlDocumentPosition::move_to_non_white(bqsql, position),
            to: BqsqlDocumentPosition::move_end(position, end),
        });
    }
    None
}

#[test]
fn test_handle_query() {
    let result = handle_query("SELECT 2+2", &BqsqlDocumentPosition::beginning_text());
    assert!(result.is_some());
}

#[test]
fn test_handle_query_after_comment() {
    let result = handle_query(
        "--super comment\nSELECT 2+2",
        &BqsqlDocumentPosition {
            column: 0,
            line: 1,
            index: 16,
        },
    );
    assert!(result.is_some());
}

#[test]
fn test_handle_query_new_line() {
    let result = handle_query("\nSELECT 2+2", &BqsqlDocumentPosition::beginning_text());
    assert!(result.is_none());
}

#[test]
fn test_handle_query_new_line_space() {
    let result = handle_query("\n SELECT 2+2", &BqsqlDocumentPosition::beginning_text());
    assert!(result.is_none());
}

#[cfg(test)]
mod tests {
    use crate::bqsql_document::{BqsqlDocument, BqsqlDocumentItemType, BqsqlDocumentType};

    #[test]
    fn empty_string() {
        let document = BqsqlDocument::parse("");

        assert_eq!(BqsqlDocumentType::UNKNOWN, document.document_type);
        assert_eq!(0, document.items.len());
    }

    #[test]
    fn comment_only() {
        let document = BqsqlDocument::parse("--super comment");

        assert_eq!(BqsqlDocumentType::UNKNOWN, document.document_type);
        assert_eq!(1, document.items.len());
        assert_eq!(BqsqlDocumentItemType::COMMENT, document.items[0].item_type);
        assert_eq!(0, document.items[0].from.column);
        assert_eq!(0, document.items[0].from.line);
        assert_eq!(0, document.items[0].from.index);
        assert_eq!(14, document.items[0].to.column);
        assert_eq!(0, document.items[0].to.line);
        assert_eq!(14, document.items[0].to.index);
    }

    #[test]
    fn comment_with_select_in_text() {
        let document = BqsqlDocument::parse("--super comment that includes a query SELECT 2+2");

        assert_eq!(BqsqlDocumentType::UNKNOWN, document.document_type);
        assert_eq!(1, document.items.len());
        assert_eq!(BqsqlDocumentItemType::COMMENT, document.items[0].item_type);
        assert_eq!(0, document.items[0].from.column);
        assert_eq!(0, document.items[0].from.line);
        assert_eq!(0, document.items[0].from.index);
        assert_eq!(47, document.items[0].to.column);
        assert_eq!(0, document.items[0].to.line);
        assert_eq!(47, document.items[0].to.index);
    }

    #[test]
    fn space_comment_only() {
        let document = BqsqlDocument::parse("    --super comment");

        assert_eq!(BqsqlDocumentType::UNKNOWN, document.document_type);
        assert_eq!(1, document.items.len());
        assert_eq!(BqsqlDocumentItemType::COMMENT, document.items[0].item_type);
        assert_eq!(4, document.items[0].from.column);
        assert_eq!(0, document.items[0].from.line);
        assert_eq!(4, document.items[0].from.index);
        assert_eq!(18, document.items[0].to.column);
        assert_eq!(0, document.items[0].to.line);
        assert_eq!(18, document.items[0].to.index);
    }

    #[test]
    fn tiny_query() {
        let document = BqsqlDocument::parse("SELECT 2+2");

        assert_eq!(BqsqlDocumentType::QUERY, document.document_type);
        assert_eq!(1, document.items.len());
        assert_eq!(BqsqlDocumentItemType::QUERY, document.items[0].item_type);
        assert_eq!(0, document.items[0].from.column);
        assert_eq!(0, document.items[0].from.line);
        assert_eq!(0, document.items[0].from.index);
        // assert_eq!(10, document.items[0].to.column);
        // assert_eq!(0, document.items[0].to.line);
        // assert_eq!(10, document.items[0].to.index);
    }

    #[test]
    fn tiny_query_second_line() {
        let document = BqsqlDocument::parse("\nSELECT 2+2");

        assert_eq!(BqsqlDocumentType::QUERY, document.document_type);
        assert_eq!(1, document.items.len());
        assert_eq!(BqsqlDocumentItemType::QUERY, document.items[0].item_type);
        assert_eq!(0, document.items[0].from.column);
        assert_eq!(1, document.items[0].from.line);
        assert_eq!(1, document.items[0].from.index);
        // assert_eq!(16, document.items[0].to.column);
        // assert_eq!(1, document.items[0].to.line);
        // assert_eq!(16, document.items[0].to.index);
    }

    #[test]
    fn comment_and_tiny_query() {
        let document = BqsqlDocument::parse("--super comment\nSELECT 2+2");

        assert_eq!(BqsqlDocumentType::QUERY, document.document_type);
        assert_eq!(2, document.items.len());
        assert_eq!(BqsqlDocumentItemType::COMMENT, document.items[0].item_type);
        assert_eq!(0, document.items[0].from.column);
        assert_eq!(0, document.items[0].from.line);
        assert_eq!(0, document.items[0].from.index);
        assert_eq!(14, document.items[0].to.column);
        assert_eq!(0, document.items[0].to.line);
        assert_eq!(14, document.items[0].to.index);
        assert_eq!(BqsqlDocumentItemType::QUERY, document.items[1].item_type);
        assert_eq!(0, document.items[1].from.column);
        assert_eq!(1, document.items[1].from.line);
        assert_eq!(16, document.items[1].from.index);
        // assert_eq!(0, document.items[1].to.column);
        // assert_eq!(1, document.items[1].to.line);
        // assert_eq!(16, document.items[1].to.index);
    }
}
