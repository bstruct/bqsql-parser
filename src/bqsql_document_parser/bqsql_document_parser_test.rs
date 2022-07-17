// #[cfg(test)]
// mod bqsql_document_parser_test {
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
// }
