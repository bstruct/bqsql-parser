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

    fn handle_with(&mut self) -> Option<BqsqlDocumentItem> {
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

            return Some(BqsqlDocumentItem::new(BqsqlDocumentItemType::Query, items));
        }
        None
    }

    fn handle_select(&mut self) -> Option<BqsqlDocumentItem> {
        if self.is_keyword(BqsqlKeyword::Select) {
            let mut items = [
                self.get_select_keywords(), //SELECT ALL, DISTINCT, AS VALUE, AS STRUCT
                self.get_select_list(),     //list of columns or calculated values
            ]
            .concat();

            items.append(&mut Vec::from(vec![
                self.handle_from(), //FROM
                                    //WHERE
                                    //GROUP BY | ROLLUP
                                    //HAVING
                                    //QUALIFY
                                    //WINDOW
                                    //ORDER BY
                                    //LIMIT
                                    //OFFSET
            ]));

            return Some(BqsqlDocumentItem::new(BqsqlDocumentItemType::Query, items));
        }
        None
    }

    fn handle_from(&mut self) -> Option<BqsqlDocumentItem> {
        if self.is_keyword(BqsqlKeyword::From) {
            todo!()
        }
        None
    }

    fn get_select_keywords(&mut self) -> Vec<Option<BqsqlDocumentItem>> {
        let mut items = Vec::from(vec![self.handle_keyword(BqsqlKeyword::Select)]);
        if self.is_keyword(BqsqlKeyword::All) {
            items.push(self.handle_keyword(BqsqlKeyword::All));
        } else if self.is_keyword(BqsqlKeyword::Distinct) {
            items.push(self.handle_keyword(BqsqlKeyword::Distinct));
        } else if self.is_keyword(BqsqlKeyword::As) {
            items.push(self.handle_keyword(BqsqlKeyword::As));
            if self.is_keyword(BqsqlKeyword::Struct) {
                items.push(self.handle_keyword(BqsqlKeyword::Struct));
            }
            if self.is_keyword(BqsqlKeyword::Value) {
                items.push(self.handle_keyword(BqsqlKeyword::Value));
            }
        }

        items
    }

    fn get_select_list(&mut self) -> Vec<Option<BqsqlDocumentItem>> {
        todo!()
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

    fn handle_open_parentheses(&mut self) -> Option<BqsqlDocumentItem> {
        self.handle_string(BqsqlDocumentItemType::ParenthesesOpen, "(")
    }

    fn handle_close_parentheses(&mut self) -> Option<BqsqlDocumentItem> {
        self.handle_string(BqsqlDocumentItemType::ParenthesesClose, ")")
    }

    fn handle_comma(&mut self) -> Option<BqsqlDocumentItem> {
        self.handle_string(BqsqlDocumentItemType::Comma, ",")
    }
}
