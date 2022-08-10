use bqsql_document::BqsqlDocument;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::bqsql_function::BqsqlFunction;

mod bqsql_document;
mod bqsql_function;
// pub mod bqsql_document_parser;
// pub mod bqsql_document::bqsql_document_parser;

#[wasm_bindgen]
pub fn parse(bqsql: &str) -> JsValue {
    let document = BqsqlDocument::parse(bqsql);

    // // wasm_bindgen::JsValue::from_serde(&document).unwrap()
    serde_wasm_bindgen::to_value(&document).unwrap()
}

#[wasm_bindgen]
pub fn get_all_functions() -> JsValue {
    serde_wasm_bindgen::to_value(&BqsqlFunction::get_all()).unwrap()
}
