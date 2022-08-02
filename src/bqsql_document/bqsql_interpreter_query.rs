use super::{
    bqsql_interpreter::BqsqlInterpreter, bqsql_keyword::BqsqlKeyword, BqsqlDocumentItem,
    BqsqlDocumentItemType,
};

impl BqsqlInterpreter<'_> {
    pub(crate) fn handle_query(&mut self) -> Option<BqsqlDocumentItem> {
        if self.is_keyword(BqsqlKeyword::With) || self.is_keyword(BqsqlKeyword::Select) {
            let document_item = BqsqlDocumentItem::new(
                BqsqlDocumentItemType::Query,
                vec![
                    self.handle_with(),   //expected possible "WITH"
                    self.handle_select(), //expected mandatory select statement
                ],
            );

            return Some(document_item);
        }

        None
    }

    pub(crate) fn handle_with(&mut self) -> Option<BqsqlDocumentItem> {
        if self.is_keyword(BqsqlKeyword::With) {
            let mut items = Vec::from(vec![
                self.handle_keyword(BqsqlKeyword::With),      //WITH
                self.handle_keyword(BqsqlKeyword::Recursive), //RECURSIVE?
            ]);

            loop {
                items.append(&mut Vec::from(vec![
                    self.handle_cte_name(),                //common table expression (CTE)
                    self.handle_keyword(BqsqlKeyword::As), //AS
                    self.handle_open_parentheses(),
                    self.handle_select(), //expected mandatory select statement
                    self.handle_close_parentheses(),
                    self.handle_comma(),
                ]));

                if items.last().is_none() {
                    break;
                }
            }

            return Some(BqsqlDocumentItem::new(
                BqsqlDocumentItemType::Query,
                items,
            ));
        }
        None
    }
    pub(crate) fn handle_select(&self) -> Option<BqsqlDocumentItem> {
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
        None
    }
    pub(crate) fn handle_cte_name(&mut self) -> Option<BqsqlDocumentItem> {
        // if self.is_keyword(BqsqlKeyword::With) {
        //     let document_item = BqsqlDocumentItem::new(
        //         BqsqlDocumentItemType::Query,
        //         vec![
        //             self.handle_keyword(BqsqlKeyword::With),      //WITH
        //             self.handle_keyword(BqsqlKeyword::Recursive), //RECURSIVE?
        //             self.handle_cte_name(),//common table expression (CTE)
        //             self.handle_keyword(BqsqlKeyword::As), //AS
        //             self.handle_open_parentheses(),
        //             self.handle_select(), //expected mandatory select statement
        //             self.handle_close_parentheses(),
        //             self.handle_comma(),
        //         ],
        //     );

        //     return Some(document_item);
        // }
        None
    }

    pub(crate) fn handle_open_parentheses(&mut self) -> Option<BqsqlDocumentItem> {
        self.handle_string(BqsqlDocumentItemType::ParenthesesOpen, "(")
    }

    pub(crate) fn handle_close_parentheses(&mut self) -> Option<BqsqlDocumentItem> {
        self.handle_string(BqsqlDocumentItemType::ParenthesesClose, ")")
    }

    pub(crate) fn handle_comma(&mut self) -> Option<BqsqlDocumentItem> {
        self.handle_string(BqsqlDocumentItemType::Comma, ",")
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
