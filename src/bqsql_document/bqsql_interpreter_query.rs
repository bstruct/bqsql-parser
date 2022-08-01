use super::{
    bqsql_interpreter::{BqsqlInterpreter, BqsqlInterpreterItems},
    bqsql_keyword::BqsqlKeyword,
    BqsqlDocumentItem, BqsqlDocumentItemType,
};

impl<'a> BqsqlInterpreter<'a> {
    pub(crate) fn handle_query(&'a mut self) -> Option<BqsqlDocumentItem> {
        if self.is_keyword(BqsqlKeyword::With) || self.is_keyword(BqsqlKeyword::Select) {
            
            // let interpreter = self;
            
            // let interpreter_items = BqsqlInterpreterItems::new(self);

            let interpreter_items2 = self.new_items();
            // // let interpreter = &interpreter_items.interpreter;
            // // let items = &interpreter_items.items;

            let _ = interpreter_items2.collect(BqsqlDocumentItemType::Alias);

            // let item = 
            //     interpreter_items
            //     // .handle_with() //expected possible "WITH"
            //     // .handle_select() //expected mandatory ( to be query ) "SELECT"
            //     .collect(BqsqlDocumentItemType::Query)
            // ;

            // return Some(item);
        }

        None
    }
}

impl<'i> BqsqlInterpreterItems<'i> {
    pub(crate) fn handle_with(&'i mut self) -> &BqsqlInterpreterItems<'i> {
        
        // let interpreter = self.interpreter;

        if self.is_keyword(BqsqlKeyword::With) {
            // let mut interpreter_items = self.interpreter.new_interpreter_items();

            // interpreter_items
            //     .handle_keyword(BqsqlKeyword::With)
            //     .handle_select()
            //     .collect_and_append(BqsqlDocumentItemType::QueryWith);

            // let with_document_item = BqsqlDocumentItem {
            //     item_type: BqsqlDocumentItemType::QueryWith,
            //     range: None,
            //     items: vec![],
            // };

            // self.append(with_document_item);

            //             // let with_document_item = &self
            //             //     .interpreter
            //             //     .new_document_item(BqsqlDocumentItemType::QueryWith);

            //             self.document_item.push(with_document_item);

            //             // self.handle_cte_name(&mut document_item_with)
            //             //     .handle_keyword_as()
            //             //     .handle_open_parentheses()
            //             //     .handle_select()
            //             //     .handle_close_parentheses()
            //             //     .handle_comma();
            //             // return Some(with_document_item);
        }
        self
    }
    pub(crate) fn handle_select(&'i mut self) -> &BqsqlInterpreterItems<'i> {
        // let interpreter = self.interpreter;

        if self.is_keyword(BqsqlKeyword::Select) {

            // interpreter
            //     .new_interpreter_items()
            //     .handle_keyword(BqsqlKeyword::Select)
            //     .handle_select()
            //     .collect_and_append(BqsqlDocumentItemType::QueryWith);

            // let with_document_item = BqsqlDocumentItem {
            //     item_type: BqsqlDocumentItemType::QuerySelect,
            //     range: None,
            //     items: vec![],
            // };

            // self.append(with_document_item);

            //             // let with_document_item = &self
            //             //     .interpreter
            //             //     .new_document_item(BqsqlDocumentItemType::QueryWith);

            //             self.document_item.push(with_document_item);

            //             // self.handle_cte_name(&mut document_item_with)
            //             //     .handle_keyword_as()
            //             //     .handle_open_parentheses()
            //             //     .handle_select()
            //             //     .handle_close_parentheses()
            //             //     .handle_comma();
            //             // return Some(with_document_item);
        }
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
