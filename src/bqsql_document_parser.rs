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

        //find the position of the word "SELECT"

        //const or simple calculation as type, example: "hi" AS name, or 1 AS number, or 2+2 as another_number  
        //column with potential alias


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
mod test_select;