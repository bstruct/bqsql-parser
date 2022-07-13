use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct BqsqlDocument {
    pub document_type: BqsqlDocumentType,
    // pub items: BqsqlDocumentItem,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BqsqlDocumentType {
    UNKNOWN = 0,
    QUERY = 1,
}

// #[wasm_bindgen]
// #[derive(Debug, Clone, Copy)]
// pub struct BqsqlDocumentItem {
//     pub item_type: BqsqlDocumentItemType,
// }

// #[wasm_bindgen]
// #[derive(Debug, Clone, Copy)]
// pub enum BqsqlDocumentItemType {
//     UNKNOWN = 0,
//     COMMENT = 1,
//     QUERY = 2,
// }
