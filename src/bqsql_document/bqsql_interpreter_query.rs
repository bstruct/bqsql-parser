use super::{bqsql_interpreter::BqsqlInterpreter, bqsql_keyword::BqsqlKeyword, BqsqlDocumentItemType};

impl BqsqlInterpreter<'_> {
    pub(crate) fn handle_query(&mut self) -> &BqsqlInterpreter {
        if self.is_keyword_with() || self.is_keyword_select() {
            //if it's the top level of the document, then signal that this block is a "Query".
            //all subsequent items will be placed nested inside
            if self.is_top_node() {
                self.new_parent_document_item(BqsqlDocumentItemType::Query);
            }

            // self
            //     .handle_with() //expected possible "WITH"
            //     .handle_select() //expected mandatory ( to be query ) "SELECT"
            //     ;
        }

        self
    }

    pub(crate) fn is_keyword_with(&self) -> bool {
        if let Some(string_in_range) = self.get_string_in_range(self.index) {
            return string_in_range == BqsqlKeyword::With;
        }
        false
    }

    pub(crate) fn is_keyword_select(&self) -> bool {
        if let Some(string_in_range) = self.get_string_in_range(self.index) {
            return string_in_range == BqsqlKeyword::Select;
        }
        false
    }

    pub(crate) fn handle_with(&self) -> &BqsqlInterpreter {
        if self.is_keyword_with() {
            // self.handle_new_node(BqsqlDocumentItemType::QueryWith)
            //     .handle_cte_name()
            //     .handle_keyword_as()
            //     .handle_open_parentheses()
            //     .handle_select()
            //     .handle_close_parentheses()
            //     .handle_comma();
        }
        self
    }
    pub(crate) fn handle_select(&self) -> &BqsqlInterpreter {
        if self.is_keyword_select() {}
        self
    }
}

// fn handle_query_resolve_select<'a>(
//     lines: &Vec<&'a str>,
//     tokens: &[[usize; 3]],
//     index: usize,
// ) -> (BqsqlDocumentItem, usize) {
//     if tokens.len() > index + 1 {
//         if let Some(string_in_range) = get_string_in_range(lines, &tokens[index + 1]) {
//             let string_in_range_upper = string_in_range.to_uppercase();

//             //
//             //QuerySelectAll
//             let select_all = string_in_range_upper == "ALL";
//             if select_all {
//                 let item = BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::QuerySelectAll,
//                     range: None,
//                     items: vec![
//                         BqsqlDocumentItem {
//                             item_type: BqsqlDocumentItemType::Keyword,
//                             range: Some(tokens[index]),
//                             items: vec![],
//                         },
//                         BqsqlDocumentItem {
//                             item_type: BqsqlDocumentItemType::Keyword,
//                             range: Some(tokens[index + 1]),
//                             items: vec![],
//                         },
//                     ],
//                 };

//                 return (item, index + 2);
//             }

//             //
//             //QuerySelectDistinct
//             let select_distinct = string_in_range_upper == "DISTINCT";
//             if select_distinct {
//                 let item = BqsqlDocumentItem {
//                     item_type: BqsqlDocumentItemType::QuerySelectDistinct,
//                     range: None,
//                     items: vec![
//                         BqsqlDocumentItem {
//                             item_type: BqsqlDocumentItemType::Keyword,
//                             range: Some(tokens[index]),
//                             items: vec![],
//                         },
//                         BqsqlDocumentItem {
//                             item_type: BqsqlDocumentItemType::Keyword,
//                             range: Some(tokens[index + 1]),
//                             items: vec![],
//                         },
//                     ],
//                 };

//                 return (item, index + 2);
//             }
//         }
//     }

//     if tokens.len() > index + 2 {
//         if let Some(string_in_range_1) = get_string_in_range(lines, &tokens[index + 1]) {
//             if let Some(string_in_range_2) = get_string_in_range(lines, &tokens[index + 2]) {
//                 let string_in_range_1_upper = string_in_range_1.to_uppercase();
//                 let string_in_range_2_upper = string_in_range_2.to_uppercase();

//                 //
//                 //QuerySelectAsStruct
//                 let select_as_struct =
//                     string_in_range_1_upper == "AS" && string_in_range_2_upper == "STRUCT";
//                 if select_as_struct {
//                     let item = BqsqlDocumentItem {
//                         item_type: BqsqlDocumentItemType::QuerySelectAsStruct,
//                         range: None,
//                         items: vec![
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index]),
//                                 items: vec![],
//                             },
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index + 1]),
//                                 items: vec![],
//                             },
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index + 2]),
//                                 items: vec![],
//                             },
//                         ],
//                     };

//                     return (item, index + 3);
//                 }

//                 //
//                 //QuerySelectAsValue
//                 let select_as_value =
//                     string_in_range_1_upper == "AS" && string_in_range_2_upper == "VALUE";
//                 if select_as_value {
//                     let item = BqsqlDocumentItem {
//                         item_type: BqsqlDocumentItemType::QuerySelectAsValue,
//                         range: None,
//                         items: vec![
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index]),
//                                 items: vec![],
//                             },
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index + 1]),
//                                 items: vec![],
//                             },
//                             BqsqlDocumentItem {
//                                 item_type: BqsqlDocumentItemType::Keyword,
//                                 range: Some(tokens[index + 2]),
//                                 items: vec![],
//                             },
//                         ],
//                     };

//                     return (item, index + 4);
//                 }
//             }
//         }
//     }
//     //QuerySelect
//     let item = BqsqlDocumentItem {
//         item_type: BqsqlDocumentItemType::QuerySelect,
//         range: None,
//         items: vec![BqsqlDocumentItem {
//             item_type: BqsqlDocumentItemType::Keyword,
//             range: Some(tokens[index]),
//             items: vec![],
//         }],
//     };

//     return (item, index + 1);
// }
