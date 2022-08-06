use super::{
    bqsql_delimiter::BqsqlDelimiter,
    bqsql_interpreter::{self, get_relevant_keywords_match, handle_semicolon, BqsqlInterpreter, is_number},
    bqsql_keyword::BqsqlKeyword,
    bqsql_query_structure::BqsqlQueryStructure,
    BqsqlDocumentItem, BqsqlDocumentItemType,
};

impl BqsqlInterpreter<'_> {
    pub(crate) fn handle_query(&mut self) -> Option<BqsqlDocumentItem> {
        if is_query(self) {
            let document_item = BqsqlDocumentItem::new(
                BqsqlDocumentItemType::Query,
                vec![
                    handle_query_stage(self, BqsqlQueryStructure::With),
                    handle_query_stage(self, BqsqlQueryStructure::Select),
                    handle_query_stage(self, BqsqlQueryStructure::From),
                    handle_query_stage(self, BqsqlQueryStructure::Where),
                    handle_query_stage(self, BqsqlQueryStructure::GroupBy),
                    handle_query_stage(self, BqsqlQueryStructure::Rollup),
                    handle_query_stage(self, BqsqlQueryStructure::Having),
                    handle_query_stage(self, BqsqlQueryStructure::Qualify),
                    handle_query_stage(self, BqsqlQueryStructure::Window),
                    handle_query_stage(self, BqsqlQueryStructure::OrderBy),
                    handle_query_stage(self, BqsqlQueryStructure::Limit),
                    handle_query_stage(self, BqsqlQueryStructure::Offset),
                    handle_semicolon(self),
                ],
            );

            return Some(document_item);
        }
        None
    }
}

// fn handle_with(&mut self) -> Option<BqsqlDocumentItem> {
//     if self.is_keyword(self.index, BqsqlKeyword::With) {
//         let mut items = Vec::from(vec![
//             self.handle_keyword(BqsqlKeyword::With),      //WITH
//             self.handle_keyword(BqsqlKeyword::Recursive), //RECURSIVE?
//         ]);

//         let mut monitor_index = self.index;

//         loop {
//             items.append(&mut Vec::from(vec![
//                 self.handle_cte_name(), //common table expression (CTE)
//                 self.handle_keyword(BqsqlKeyword::As),
//                 self.handle_delimiter(BqsqlDelimiter::ParenthesesOpen),
//                 self.handle_query(), //expected mandatory select statement
//                 self.handle_delimiter(BqsqlDelimiter::ParenthesesClose),
//                 self.handle_delimiter(BqsqlDelimiter::Comma),
//             ]));

//             if monitor_index == self.index {
//                 break;
//             } else {
//                 monitor_index = self.index;
//             }

//             if items.last().is_none() {
//                 break;
//             }
//         }

//         return Some(BqsqlDocumentItem::new(
//             BqsqlDocumentItemType::QueryWith,
//             items,
//         ));
//     }
//     None
// }

// fn get_select_list(&mut self) -> Vec<Option<BqsqlDocumentItem>> {
//     //list to return with the found BqsqlDocumentItemType::QuerySelectListItem
//     let mut list: Vec<Option<BqsqlDocumentItem>> = Vec::new();
//     //gather all elements inside the current BqsqlDocumentItemType::QuerySelectListItem
//     let mut item_list: Vec<Option<BqsqlDocumentItem>> = Vec::new();

//     let mut monitor_index = self.index;

//     let mut count_open_parentheses: usize = 0;

//     loop {
//         //line comment
//         if self.is_line_comment(self.index) {
//             item_list.push(self.handle_document_item(BqsqlDocumentItemType::LineComment));
//         }

//         //query
//         if self.is_query() {
//             item_list.push(self.handle_query());
//         }

//         if self.is_delimiter(self.index, BqsqlDelimiter::ParenthesesOpen) {
//             //parentheses open
//             item_list.push(self.handle_document_item(BqsqlDocumentItemType::ParenthesesOpen));
//             count_open_parentheses += 1;
//         } else if self.is_delimiter(self.index, BqsqlDelimiter::ParenthesesClose) {
//             //parentheses close
//             if count_open_parentheses == 0 {
//                 break;
//             }
//             item_list.push(self.handle_document_item(BqsqlDocumentItemType::ParenthesesClose));
//             count_open_parentheses = std::cmp::max(0, count_open_parentheses - 1);
//             continue;
//         } else if self.is_number() {
//             //number
//             item_list.push(self.handle_document_item(BqsqlDocumentItemType::Number));
//         } else if self.is_string(self.index) {
//             //string
//             item_list.push(self.handle_document_item(BqsqlDocumentItemType::String));
//         } else if self.is_keyword(self.index, BqsqlKeyword::As) {
//             //keyword AS
//             item_list.push(self.handle_document_item(BqsqlDocumentItemType::KeywordAs));
//         } else if self.is_delimiter(self.index, BqsqlDelimiter::Comma) {
//             //comma
//             item_list.push(self.handle_document_item(BqsqlDocumentItemType::Comma));
//         } else if let Some(operators) = self.find_any_operator(self.index) {
//             //any operator
//             let mut len = operators.to_vec().len();
//             while len > 0 {
//                 item_list.push(self.handle_document_item(BqsqlDocumentItemType::Operator));
//                 len -= 1;
//             }
//         } else if count_open_parentheses == 0 {
//             if !self.is_keyword(self.index, BqsqlKeyword::From) {
//                 item_list.push(self.handle_document_item(BqsqlDocumentItemType::Alias));
//             }
//         } else {
//             item_list.push(self.handle_unknown());
//         }

//         if monitor_index == self.index {
//             break;
//         } else {
//             monitor_index = self.index;
//         }

//         if count_open_parentheses == 0
//             && item_list.len() > 0
//             && item_list.last().is_some()
//             // .is_some_and(|i| true) // is_some_and is not complete at the moment
//             && self.is_delimiter(self.index - 1, BqsqlDelimiter::Comma)
//         {
//             list.push(Some(BqsqlDocumentItem::new(
//                 BqsqlDocumentItemType::QuerySelectListItem,
//                 item_list,
//             )));

//             item_list = Vec::new();
//             continue;
//         }

//         if (!self.is_in_range(self.index)) || self.is_keyword(self.index, BqsqlKeyword::From) {
//             break;
//         }
//     }

//     if item_list.len() > 0 {
//         list.push(Some(BqsqlDocumentItem::new(
//             BqsqlDocumentItemType::QuerySelectListItem,
//             item_list,
//         )));
//     }

//     list
// }

fn is_query(interpreter: &BqsqlInterpreter) -> bool {
    interpreter.is_keyword(interpreter.index, BqsqlKeyword::With)
        || interpreter.is_keyword(interpreter.index, BqsqlKeyword::Select)
}

fn handle_query_stage(
    interpreter: &mut BqsqlInterpreter,
    query_stage: BqsqlQueryStructure,
) -> Option<BqsqlDocumentItem> {
    match query_stage {
        BqsqlQueryStructure::With => {
            return handle_query_stage_default(
                interpreter,
                query_stage,
                document_item_handler_with,
            );
        }
        // BqsqlQueryStructure::Select => {},
        _ => {
            return handle_query_stage_default(
                interpreter,
                query_stage,
                document_item_handler_unknown,
            );
        }
    };
}

fn handle_query_stage_default(
    interpreter: &mut BqsqlInterpreter,
    query_stage: BqsqlQueryStructure,
    //https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html
    document_item_handler: fn(&mut BqsqlInterpreter) -> Option<BqsqlDocumentItem>,
) -> Option<BqsqlDocumentItem> {
    //get keywords associated with this query stage to
    // 1) confirm that they are found
    // 2) if they are found, they are added as keywords to the document_item items
    let keywords_to_match = &query_stage.get_keywords();
    if let Some(keywords_match) =
        bqsql_interpreter::get_relevant_keywords_match(interpreter, keywords_to_match)
    {
        let mut items: Vec<Option<BqsqlDocumentItem>> = Vec::new();

        //add keywords
        let mut matched = 0;
        while matched < keywords_match.len() {
            if interpreter.is_line_comment(interpreter.index) {
                items.push(interpreter.handle_document_item(BqsqlDocumentItemType::LineComment));
            } else {
                items.push(interpreter.handle_document_item(BqsqlDocumentItemType::Keyword));
                matched += 1;
            }
        }

        let subsequent_query_structure = &query_stage.get_subsequent_query_structure();

        //get items
        let new_items = &mut loop_query_default(
            interpreter,
            subsequent_query_structure,
            document_item_handler,
        );
        items.append(new_items);

        return Some(BqsqlDocumentItem::new(
            query_stage.get_document_item_type(),
            items,
        ));
    }
    None
}

fn loop_query_default(
    interpreter: &mut BqsqlInterpreter,
    subsequent_query_structure: &Vec<BqsqlQueryStructure>,
    document_item_handler: fn(&mut BqsqlInterpreter) -> Option<BqsqlDocumentItem>,
) -> Vec<Option<BqsqlDocumentItem>> {
    let mut items: Vec<Option<BqsqlDocumentItem>> = Vec::new();

    //loop tokens inside the expected block of the query
    let mut count_open_parentheses: usize = 0;
    while continue_loop_query(
        interpreter,
        subsequent_query_structure,
        count_open_parentheses,
    ) {
        if interpreter.is_delimiter(interpreter.index, BqsqlDelimiter::ParenthesesOpen) {
            //parentheses open
            items.push(interpreter.handle_document_item(BqsqlDocumentItemType::ParenthesesOpen));
            count_open_parentheses += 1;
            continue;
        } else if interpreter.is_delimiter(interpreter.index, BqsqlDelimiter::ParenthesesClose) {
            //parentheses close
            items.push(interpreter.handle_document_item(BqsqlDocumentItemType::ParenthesesClose));
            count_open_parentheses = std::cmp::max(0, count_open_parentheses - 1);
            continue;
        }  else if interpreter.is_delimiter(interpreter.index, BqsqlDelimiter::Comma) {
            //comma
            items.push(interpreter.handle_document_item(BqsqlDocumentItemType::Comma));
            continue;
        } else if is_query(interpreter) {
            items.push(interpreter.handle_query());
            continue;
        } else if is_number(interpreter) {
            //number
            items.push(interpreter.handle_document_item(BqsqlDocumentItemType::Number));
            continue;
        } else if interpreter.is_string(interpreter.index) {
            //string
            items.push(interpreter.handle_document_item(BqsqlDocumentItemType::String));
            continue;
        } else if interpreter.is_line_comment(interpreter.index) {
            //line comment
            items.push(interpreter.handle_document_item(BqsqlDocumentItemType::LineComment));
            continue;
        }

        items.push(document_item_handler(interpreter));
    }

    items
}

fn document_item_handler_unknown(interpreter: &mut BqsqlInterpreter) -> Option<BqsqlDocumentItem> {
    interpreter.handle_document_item(BqsqlDocumentItemType::Unknown)
}

fn document_item_handler_with(interpreter: &mut BqsqlInterpreter) -> Option<BqsqlDocumentItem> {
    if interpreter.is_keyword(interpreter.index - 1, BqsqlKeyword::With)
        || interpreter.is_delimiter(interpreter.index - 1, BqsqlDelimiter::Comma)
    {
        return interpreter.handle_document_item(BqsqlDocumentItemType::QueryCteName);
    }
    if interpreter.is_keyword(interpreter.index, BqsqlKeyword::As) {
        return interpreter.handle_document_item(BqsqlDocumentItemType::KeywordAs);
    }
    if interpreter.is_keyword(interpreter.index - 1, BqsqlKeyword::As) {
        return interpreter.handle_document_item(BqsqlDocumentItemType::Alias);
    }

    interpreter.handle_document_item(BqsqlDocumentItemType::Unknown)
}

fn continue_loop_query(
    interpreter: &BqsqlInterpreter,
    subsequent_query_structure: &Vec<BqsqlQueryStructure>,
    count_open_parentheses: usize,
) -> bool {
    if interpreter.is_in_range(interpreter.index) {
        //;
        if interpreter.is_delimiter(interpreter.index, BqsqlDelimiter::Semicolon) {
            return false;
        }

        //parentheses close
        if interpreter.is_delimiter(interpreter.index, BqsqlDelimiter::ParenthesesClose) {
            if count_open_parentheses == 0 {
                return false;
            }
        }

        if count_open_parentheses > 0 {
            return true;
        }

        //are any of the subsequent keywords of the query found?
        return !subsequent_query_structure
            .iter()
            .map(|i| i.get_keywords())
            .any(|i| get_relevant_keywords_match(&interpreter, &i).is_some());
    }

    false
}

#[test]
fn continue_loop_query_from_where_not_continue() {
    let mut interpreter = BqsqlInterpreter::new("SELECT 1 FROM t WHERE 1=1");
    interpreter.index = 4;
    let query_stage: BqsqlQueryStructure = BqsqlQueryStructure::From;

    assert!(!continue_loop_query(
        &interpreter,
        &query_stage.get_subsequent_query_structure(),
        0
    ));
}

#[test]
fn continue_loop_query_from_where_continue() {
    let mut interpreter = BqsqlInterpreter::new("SELECT 1 FROM dataset.table WHERE 1=1");
    interpreter.index = 4;
    let query_stage: BqsqlQueryStructure = BqsqlQueryStructure::From;

    assert!(continue_loop_query(
        &interpreter,
        &query_stage.get_subsequent_query_structure(),
        0
    ));
}

#[test]
fn continue_loop_query_from_end() {
    let mut interpreter = BqsqlInterpreter::new("SELECT 1 FROM dataset.table");
    interpreter.index = 6;
    let query_stage: BqsqlQueryStructure = BqsqlQueryStructure::From;

    assert!(!continue_loop_query(
        &interpreter,
        &query_stage.get_subsequent_query_structure(),
        0
    ));
}

#[test]
fn continue_loop_query_with() {
    let mut interpreter = BqsqlInterpreter::new(
        r#"WITH q1 AS (SELECT SchoolID FROM Roster) #my_query
SELECT *
FROM
(WITH q2 AS (SELECT * FROM q1),  # q1 resolves to my_query
    q3 AS (SELECT * FROM q1),  # q1 resolves to my_query
    q1 AS (SELECT * FROM q1),  # q1 (in the query) resolves to my_query
    q4 AS (SELECT * FROM q1)   # q1 resolves to the WITH subquery on the previous line.
SELECT * FROM q1);             # q1 resolves to the third inner WITH subquery."#,
    );

    interpreter.index = 9;
    let query_stage: BqsqlQueryStructure = BqsqlQueryStructure::With;

    assert!(continue_loop_query(
        &interpreter,
        &query_stage.get_subsequent_query_structure(),
        0
    ));
}
