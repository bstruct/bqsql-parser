use crate::bqsql_document::BqsqlDocument;
use wasm_bindgen::prelude::*;
mod bqsql_document;
mod bqsql_document_parser;

#[wasm_bindgen]
pub fn parse(bqsql: &str) -> BqsqlDocument {
    BqsqlDocument::parse(bqsql)
}

#[cfg(test)]
mod tests {
    use crate::{bqsql_document::BqsqlDocumentType, parse};

    #[test]
    fn empty_string() {
        let response = parse(&String::from(""));

        assert_eq!(BqsqlDocumentType::UNKNOWN, response.document_type);
    }
}
