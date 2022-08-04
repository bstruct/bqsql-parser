use super::{
    bqsql_delimiter::BqsqlDelimiter, bqsql_interpreter::BqsqlInterpreter,
    bqsql_keyword::BqsqlKeyword, BqsqlDocumentItem, BqsqlDocumentItemType,
};

impl BqsqlInterpreter<'_> {
    pub(crate) fn handle_query(&mut self) -> Option<BqsqlDocumentItem> {
        if self.is_keyword(self.index, BqsqlKeyword::With)
            || self.is_keyword(self.index, BqsqlKeyword::Select)
        {
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

    fn handle_with(&mut self) -> Option<BqsqlDocumentItem> {
        if self.is_keyword(self.index, BqsqlKeyword::With) {
            let mut items = Vec::from(vec![
                self.handle_keyword(BqsqlKeyword::With),      //WITH
                self.handle_keyword(BqsqlKeyword::Recursive), //RECURSIVE?
            ]);

            loop {
                items.append(&mut Vec::from(vec![
                    self.handle_cte_name(), //common table expression (CTE)
                                            // self.handle_keyword(BqsqlKeyword::As), //AS
                                            // self.handle_open_parentheses(),
                                            // self.handle_select(), //expected mandatory select statement
                                            // self.handle_close_parentheses(),
                                            // self.handle_comma(),
                ]));

                if items.last().is_none() {
                    break;
                }
            }

            return Some(BqsqlDocumentItem::new(BqsqlDocumentItemType::Query, items));
        }
        None
    }

    fn handle_select(&mut self) -> Option<BqsqlDocumentItem> {
        if self.is_keyword(self.index, BqsqlKeyword::Select) {
            let select_keywords = self.get_select_keywords();
            let items = [
                select_keywords.1,      //SELECT ALL, DISTINCT, AS VALUE, AS STRUCT
                self.get_select_list(), //list of columns or calculated values
            ]
            .concat();

            // items.append(&mut Vec::from(vec![
            //     self.handle_from(), //FROM
            //                         //WHERE
            //                         //GROUP BY | ROLLUP
            //                         //HAVING
            //                         //QUALIFY
            //                         //WINDOW
            //                         //ORDER BY
            //                         //LIMIT
            //                         //OFFSET
            // ]));

            return Some(BqsqlDocumentItem::new(select_keywords.0, items));
        }
        None
    }

    fn get_select_keywords(&mut self) -> (BqsqlDocumentItemType, Vec<Option<BqsqlDocumentItem>>) {
        let mut document_type = BqsqlDocumentItemType::QuerySelect;
        let mut items = Vec::from(vec![self.handle_keyword(BqsqlKeyword::Select)]);
        if self.is_keyword(self.index, BqsqlKeyword::All) {
            document_type = BqsqlDocumentItemType::QuerySelectAll;
            items.push(self.handle_keyword(BqsqlKeyword::All));
        } else if self.is_keyword(self.index, BqsqlKeyword::Distinct) {
            document_type = BqsqlDocumentItemType::QuerySelectDistinct;
            items.push(self.handle_keyword(BqsqlKeyword::Distinct));
        } else if self.is_keyword(self.index, BqsqlKeyword::As) {
            items.push(self.handle_keyword(BqsqlKeyword::As));
            if self.is_keyword(self.index, BqsqlKeyword::Struct) {
                document_type = BqsqlDocumentItemType::QuerySelectAsStruct;
                items.push(self.handle_keyword(BqsqlKeyword::Struct));
            } else if self.is_keyword(self.index, BqsqlKeyword::Value) {
                document_type = BqsqlDocumentItemType::QuerySelectAsValue;
                items.push(self.handle_keyword(BqsqlKeyword::Value));
            }
        }

        (document_type, items)
    }

    fn get_select_list(&mut self) -> Vec<Option<BqsqlDocumentItem>> {
        //list to return with the found BqsqlDocumentItemType::QuerySelectListItem
        let mut list: Vec<Option<BqsqlDocumentItem>> = Vec::new();
        //gather all elements inside the current BqsqlDocumentItemType::QuerySelectListItem
        let mut item_list: Vec<Option<BqsqlDocumentItem>> = Vec::new();

        let mut monitor_index = self.index;

        let mut count_open_parentheses: usize = 0;

        loop {
            //line comment
            if self.is_line_comment() {
                item_list.push(self.handle_document_item(BqsqlDocumentItemType::LineComment));
            }

            

            if self.is_delimiter(self.index, BqsqlDelimiter::ParenthesesOpen) {
                //parentheses open
                item_list.push(self.handle_document_item(BqsqlDocumentItemType::ParenthesesOpen));
                count_open_parentheses += 1;
            } else if self.is_delimiter(self.index, BqsqlDelimiter::ParenthesesClose) {
                //parentheses close
                if count_open_parentheses == 0 {
                    break;
                }
                item_list.push(self.handle_document_item(BqsqlDocumentItemType::ParenthesesClose));
                count_open_parentheses = std::cmp::max(0, count_open_parentheses - 1);
                continue;
            }else if self.is_number() {
                //number
                item_list.push(self.handle_document_item(BqsqlDocumentItemType::Number));
            } else if self.is_string(self.index) {
                //string
                item_list.push(self.handle_document_item(BqsqlDocumentItemType::String));
            } else if self.is_keyword(self.index, BqsqlKeyword::As) {
                //keyword AS
                item_list.push(self.handle_document_item(BqsqlDocumentItemType::KeywordAs));
            } else if self.is_keyword(self.index, BqsqlKeyword::Select) {
                //keyword SELECT
                item_list.push(self.handle_query());
            } else if self.is_delimiter(self.index, BqsqlDelimiter::Comma) {
                //comma
                item_list.push(self.handle_document_item(BqsqlDocumentItemType::Comma));
            } else if let Some(operators) = self.find_any_operator(self.index) {
                //any operator
                let mut len = operators.to_vec().len();
                while len > 0 {
                    item_list.push(self.handle_document_item(BqsqlDocumentItemType::Operator));
                    len -= 1;
                }
            } else if count_open_parentheses == 0 {
                if !self.is_keyword(self.index, BqsqlKeyword::From) {
                    item_list.push(self.handle_document_item(BqsqlDocumentItemType::Alias));
                }
            }

            if monitor_index == self.index {
                break;
            } else {
                monitor_index = self.index;
            }

            if count_open_parentheses == 0
                && item_list.len() > 0
                && item_list.last().is_some()
                // .is_some_and(|i| true) // is_some_and is not complete at the moment
                && self.is_delimiter(self.index - 1, BqsqlDelimiter::Comma)
            {
                list.push(Some(BqsqlDocumentItem::new(
                    BqsqlDocumentItemType::QuerySelectListItem,
                    item_list,
                )));

                item_list = Vec::new();
                continue;
            }

            if (!self.is_in_range(self.index)) || self.is_keyword(self.index, BqsqlKeyword::From) {
                break;
            }
        }

        if item_list.len() > 0 {
            list.push(Some(BqsqlDocumentItem::new(
                BqsqlDocumentItemType::QuerySelectListItem,
                item_list,
            )));
        }

        list
    }

    fn handle_from(&mut self) -> Option<BqsqlDocumentItem> {
        if self.is_keyword(self.index, BqsqlKeyword::From) {
            todo!()
        }
        None
    }

    fn handle_cte_name(&mut self) -> Option<BqsqlDocumentItem> {
        if self.is_in_range(self.index) {
            return Some(BqsqlDocumentItem {
                item_type: BqsqlDocumentItemType::QueryCteName,
                range: Some(self.tokens[self.index]),
                items: vec![],
            });
        }
        None
    }

    // fn handle_open_parentheses(&mut self) -> Option<BqsqlDocumentItem> {
    //     self.handle_string(
    //         BqsqlDocumentItemType::ParenthesesOpen,
    //         BqsqlDelimiter::ParenthesesOpen.as_str(),
    //     )
    // }

    // fn handle_close_parentheses(&mut self) -> Option<BqsqlDocumentItem> {
    //     self.handle_string(
    //         BqsqlDocumentItemType::ParenthesesClose,
    //         BqsqlDelimiter::ParenthesesClose.as_str(),
    //     )
    // }

    // fn handle_comma(&mut self) -> Option<BqsqlDocumentItem> {
    //     self.handle_string(BqsqlDocumentItemType::Comma, BqsqlDelimiter::Comma.as_str())
    // }
}
