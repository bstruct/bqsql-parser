use crate::bqsql_document::{BqsqlDocument, BqsqlDocumentItemType};

#[test]
#[ignore = "not ready yet"]
fn queries_file() {
    let bqsql = include_str!("query_files/queries.bqsql");

    let document = BqsqlDocument::parse(bqsql);

    assert_eq!(97, document.items.len());
}
