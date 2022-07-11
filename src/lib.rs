use crate::bqsql_document::BqsqlDocument;
use wasm_bindgen::prelude::*;
mod bqsql_document;

#[wasm_bindgen]
pub fn parse(bqsql: &str) -> Option<BqsqlDocument> {
    if bqsql.len() == 0 {
        return None;
    }

    None
}
