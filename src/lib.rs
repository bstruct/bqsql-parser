use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::bqsql_document::BqsqlDocument;
mod bqsql_document;
mod bqsql_document_parser;

#[wasm_bindgen]
pub fn parse(bqsql: &str) -> JsValue {
    let document = BqsqlDocument::parse(bqsql);

    // wasm_bindgen::JsValue::from_serde(&document).unwrap()
    serde_wasm_bindgen::to_value(&document).unwrap()
}