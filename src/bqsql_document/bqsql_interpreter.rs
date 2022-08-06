use lazy_static::lazy_static;
use regex::Regex;

use super::{
    bqsql_delimiter::BqsqlDelimiter, bqsql_keyword::BqsqlKeyword, bqsql_operator::BqsqlOperator,
    bqsql_query_structure::BqsqlQueryStructure, BqsqlDocumentItem, BqsqlDocumentItemType,
};
use crate::bqsql_document::token_parser;

pub(crate) struct BqsqlInterpreter<'a> {
    pub(crate) lines: Vec<&'a str>,
    pub(crate) tokens: Vec<[usize; 3]>,
    pub(crate) index: usize,
}

impl BqsqlInterpreter<'_> {
    pub(crate) fn new(bqsql: &str) -> BqsqlInterpreter {
        let lines = bqsql.lines().map(|l| l).collect::<Vec<&str>>();
        let tokens = token_parser::parse_tokens(bqsql);

        BqsqlInterpreter {
            lines: lines,
            tokens: tokens,
            index: 0,
        }
    }

    pub(crate) fn is_keyword(&self, index: usize, keyword: BqsqlKeyword) -> bool {
        if let Some(string_in_range) = self.get_string_in_range(index) {
            return string_in_range == keyword;
        }
        false
    }

    pub(crate) fn is_line_comment(&self, index: usize) -> bool {
        if let Some(string_in_range) = self.get_string_in_range(index) {
            return string_in_range.starts_with("--") || string_in_range.starts_with("#");
        }
        false
    }

    /*most generic verstion of the handle_
    if return a BqsqlDocumentItem, moves the index by 1 */
    pub(crate) fn handle_document_item(
        &mut self,
        item_type: BqsqlDocumentItemType,
    ) -> Option<BqsqlDocumentItem> {
        if self.is_in_range(self.index) {
            let item = BqsqlDocumentItem {
                item_type: item_type,
                range: Some(self.tokens[self.index]),
                items: vec![],
            };

            self.index += 1;

            return Some(item);
        }
        None
    }

    // pub(crate) fn handle_keyword(&mut self, keyword: BqsqlKeyword) -> Option<BqsqlDocumentItem> {
    //     if self.is_keyword(self.index, keyword) {
    //         return self.handle_string(BqsqlDocumentItemType::Keyword, keyword.as_str());
    //     }
    //     None
    // }

    // pub(crate) fn handle_delimiter(
    //     &mut self,
    //     delimiter: BqsqlDelimiter,
    // ) -> Option<BqsqlDocumentItem> {
    //     if self.is_delimiter(self.index, delimiter) {
    //         let item_type: BqsqlDocumentItemType = delimiter.get_item_type();
    //         return self.handle_string(item_type, delimiter.as_str());
    //     }
    //     None
    // }

    // pub(crate) fn handle_string(
    //     &mut self,
    //     item_type: BqsqlDocumentItemType,
    //     comp: &str,
    // ) -> Option<BqsqlDocumentItem> {
    //     if let Some(string_in_range) = self.get_string_in_range(self.index) {
    //         if string_in_range == comp {
    //             return self.handle_document_item(item_type);
    //         }
    //     }
    //     None
    // }

    pub(crate) fn handle_unknown(&mut self) -> Option<BqsqlDocumentItem> {
        if self.is_in_range(self.index) {
            let item = BqsqlDocumentItem {
                item_type: BqsqlDocumentItemType::Unknown,
                range: Some(self.tokens[self.index]),
                items: vec![],
            };

            self.index += 1;

            return Some(item);
        }

        None
    }

    pub(crate) fn is_in_range(&self, index: usize) -> bool {
        self.tokens.len() > index
    }

    pub(crate) fn is_delimiter(&self, index: usize, delimiter: BqsqlDelimiter) -> bool {
        if let Some(string_in_range) = self.get_string_in_range(index) {
            if string_in_range == delimiter {
                return string_in_range == delimiter;
            }
        }
        false
    }

    pub(crate) fn is_string(&self, index: usize) -> bool {
        if let Some(string_in_range) = self.get_string_in_range(index) {
            return string_in_range.starts_with("'") || string_in_range.starts_with("\"");
        }
        false
    }

    pub(crate) fn get_string_in_range(&self, index: usize) -> Option<&'_ str> {
        if self.is_in_range(index) {
            let range = &self.tokens[index];
            if self.lines[range[0]].len() >= range[2] {
                return Some(&self.lines[range[0]][range[1]..range[2]]);
            }
        }
        None
    }

    // pub(crate) fn find_any_operator(&self, index: usize) -> Option<BqsqlOperator> {
    //     let all = &BqsqlOperator::get_all();

    //     if self.is_in_range(index + 2) {
    //         let values3 = vec![
    //             self.get_string_in_range(index),
    //             self.get_string_in_range(index + 1),
    //             self.get_string_in_range(index + 2),
    //         ]
    //         .iter()
    //         .map(|i| i.unwrap().to_string())
    //         .collect::<Vec<String>>();
    //         let mut op3 = all
    //             .iter()
    //             .map(|i| (i, i.to_vec()))
    //             .filter(|i| i.1.len() == 3);

    //         if let Some(m) = op3.find(|i| i.1.to_vec() == values3) {
    //             return Some(m.0.to_owned());
    //         }
    //     }

    //     if self.is_in_range(index + 1) {
    //         let values2 = vec![
    //             self.get_string_in_range(index),
    //             self.get_string_in_range(index + 1),
    //         ]
    //         .iter()
    //         .map(|i| i.unwrap().to_string())
    //         .collect::<Vec<String>>();
    //         let mut op2 = all
    //             .iter()
    //             .map(|i| (i, i.to_vec()))
    //             .filter(|i| i.1.len() == 2);

    //         if let Some(m) = op2.find(|i| i.1.to_vec() == values2) {
    //             return Some(m.0.to_owned());
    //         }
    //     }

    //     if self.is_in_range(index) {
    //         let values1 = vec![self.get_string_in_range(index)]
    //             .iter()
    //             .map(|i| i.unwrap().to_string())
    //             .collect::<Vec<String>>();
    //         let mut op1 = all
    //             .iter()
    //             .map(|i| (i, i.to_vec()))
    //             .filter(|i| i.1.len() == 1);

    //         if let Some(m) = op1.find(|i| i.1.to_vec() == values1) {
    //             return Some(m.0.to_owned());
    //         }
    //     }

    //     None
    // }

    pub(crate) fn collect(&mut self) -> Vec<BqsqlDocumentItem> {
        let mut monitor_index = self.index;
        let mut items: Vec<BqsqlDocumentItem> = Vec::new();

        while self.tokens.len() > self.index {
            if self.is_line_comment(self.index) {
                items.push(
                    self.handle_document_item(BqsqlDocumentItemType::LineComment)
                        .unwrap(),
                );
            }

            if let Some(query) = self.handle_query() {
                items.push(query);
            }

            if monitor_index == self.index {
                if let Some(unknown) = self.handle_unknown() {
                    items.push(unknown);
                }
            } else {
                monitor_index = self.index;
            }

            self.index += 1;
        }

        items
    }
}

pub(crate) fn is_number(interpreter: &BqsqlInterpreter) -> bool {
    if let Some(string_in_range) = interpreter.get_string_in_range(interpreter.index) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d+$|^\d*\.{1}\d*$").unwrap();
        }

        return RE.is_match(string_in_range);
    }
    false
}

#[test]
fn is_number_q1(){
    let mut interpreter = BqsqlInterpreter::new("SELECT q1");
    interpreter.index=1;
    assert_eq!(&2, &interpreter.tokens.len());
    assert!(!is_number(&interpreter));
}

#[test]
fn is_number_1(){
    let mut interpreter = BqsqlInterpreter::new("SELECT 1");
    interpreter.index=1;
    assert_eq!(&2, &interpreter.tokens.len());
    assert!(is_number(&interpreter));
}

#[test]
fn is_number_1_112(){
    let mut interpreter = BqsqlInterpreter::new("SELECT 1.112");
    interpreter.index=1;
    assert_eq!(&2, &interpreter.tokens.len());
    assert!(is_number(&interpreter));
}

#[test]
fn is_number_1_(){
    let mut interpreter = BqsqlInterpreter::new("SELECT 1.");
    interpreter.index=1;
    assert_eq!(&2, &interpreter.tokens.len());
    assert!(is_number(&interpreter));
}

#[test]
fn is_number_dot_112(){
    let mut interpreter = BqsqlInterpreter::new("SELECT .112");
    interpreter.index=1;
    assert_eq!(&2, &interpreter.tokens.len());
    assert!(is_number(&interpreter));
}

/*
try to match a sequence of keywords
the "relevant" part of the name, means that line comments in the middle will be ignored and match will still be possible
comments in the beginning will not match
 */
pub(crate) fn get_relevant_keywords_match(
    interpreter: &BqsqlInterpreter,
    keywords_to_match: &Vec<Vec<BqsqlKeyword>>,
) -> Option<Vec<BqsqlKeyword>> {
    //do not accept comments in the beginning
    if interpreter.is_line_comment(interpreter.index) {
        return None;
    }

    for keywords in keywords_to_match {
        let mut matched = 0;
        let mut index = interpreter.index;

        while interpreter.is_in_range(index) && matched < keywords.len() {
            let keyword = keywords[matched];

            if interpreter.is_line_comment(index) {
                index += 1;
            } else {
                if interpreter.is_keyword(index, keyword) {
                    index += 1;
                    matched += 1;
                } else {
                    break;
                }
            }
            if matched >= keywords.len() {
                return Some(keywords.to_owned());
            }
        }
    }

    None
}

#[test]
fn get_relevant_keywords_match_select_short() {
    let interpreter = BqsqlInterpreter::new("SELECT 1");
    let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

    let keywords_option = get_relevant_keywords_match(&interpreter, &keywords_to_match);

    assert!(keywords_option.is_some());
    let keywords = keywords_option.unwrap();
    assert_eq!(1, keywords.len());
    assert_eq!(BqsqlKeyword::Select, keywords[0]);
}

#[test]
fn get_relevant_keywords_match_select() {
    let interpreter = BqsqlInterpreter::new("SELECT 1,2,3");
    let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

    let keywords_option = get_relevant_keywords_match(&interpreter, &keywords_to_match);

    assert!(keywords_option.is_some());
    let keywords = keywords_option.unwrap();
    assert_eq!(1, keywords.len());
    assert_eq!(BqsqlKeyword::Select, keywords[0]);
}

#[test]
fn get_relevant_keywords_match_select_all() {
    let interpreter = BqsqlInterpreter::new("SELECT ALL 1,2,3");
    let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

    let keywords_option = get_relevant_keywords_match(&interpreter, &keywords_to_match);

    assert!(keywords_option.is_some());
    let keywords = keywords_option.unwrap();
    assert_eq!(2, keywords.len());
    assert_eq!(BqsqlKeyword::Select, keywords[0]);
    assert_eq!(BqsqlKeyword::All, keywords[1]);
}

#[test]
fn get_relevant_keywords_match_select_distinct() {
    let interpreter = BqsqlInterpreter::new("SELECT\n DISTINCT 1,2,3");
    let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

    let keywords_option = get_relevant_keywords_match(&interpreter, &keywords_to_match);

    assert!(keywords_option.is_some());
    let keywords = keywords_option.unwrap();
    assert_eq!(2, keywords.len());
    assert_eq!(BqsqlKeyword::Select, keywords[0]);
    assert_eq!(BqsqlKeyword::Distinct, keywords[1]);
}

#[test]
fn get_relevant_keywords_match_select_as_struct() {
    let interpreter = BqsqlInterpreter::new("SELECT AS STRUCT 1,2,3");
    let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

    let keywords_option = get_relevant_keywords_match(&interpreter, &keywords_to_match);

    assert!(keywords_option.is_some());
    let keywords = keywords_option.unwrap();
    assert_eq!(3, keywords.len());
    assert_eq!(BqsqlKeyword::Select, keywords[0]);
    assert_eq!(BqsqlKeyword::As, keywords[1]);
    assert_eq!(BqsqlKeyword::Struct, keywords[2]);
}

#[test]
fn get_relevant_keywords_match_select_as_value() {
    let interpreter = BqsqlInterpreter::new("SELECT AS\n--jsafkljsafd\n VALUE 1,2,3");
    let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

    let keywords_option = get_relevant_keywords_match(&interpreter, &keywords_to_match);

    assert!(keywords_option.is_some());
    let keywords = keywords_option.unwrap();
    assert_eq!(3, keywords.len());
    assert_eq!(BqsqlKeyword::Select, keywords[0]);
    assert_eq!(BqsqlKeyword::As, keywords[1]);
    assert_eq!(BqsqlKeyword::Value, keywords[2]);
}

#[test]
fn get_relevant_keywords_match_from() {
    let mut interpreter = BqsqlInterpreter::new("SELECT AS\n--jsafkljsafd\n VALUE 1,2,3 FROM a");
    interpreter.index = 9;
    let keywords_to_match = BqsqlQueryStructure::From.get_keywords();

    let keywords_option = get_relevant_keywords_match(&interpreter, &keywords_to_match);

    assert!(keywords_option.is_some());
    let keywords = keywords_option.unwrap();
    assert_eq!(1, keywords.len());
    assert_eq!(BqsqlKeyword::From, keywords[0]);
}

#[test]
fn get_relevant_keywords_match_where() {
    let mut interpreter =
        BqsqlInterpreter::new("SELECT AS\n#jsa fkl jsafd\n VALUE 1,2,3 FROM a WHERE 1=1");
    interpreter.index = 11;
    let keywords_to_match = BqsqlQueryStructure::Where.get_keywords();

    let keywords_option = get_relevant_keywords_match(&interpreter, &keywords_to_match);

    assert!(keywords_option.is_some());
    let keywords = keywords_option.unwrap();
    assert_eq!(1, keywords.len());
    assert_eq!(BqsqlKeyword::Where, keywords[0]);
}

#[test]
fn get_relevant_keywords_match_with() {
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
    let keywords_to_match = BqsqlQueryStructure::Select.get_keywords();

    let keywords_option = get_relevant_keywords_match(&interpreter, &keywords_to_match);

    assert!(keywords_option.is_none());
}

pub(crate) fn handle_semicolon(interpreter: &mut BqsqlInterpreter) -> Option<BqsqlDocumentItem> {
    //do not accept comments in the beginning
    if let Some(string_in_range) = interpreter.get_string_in_range(interpreter.index) {
        if string_in_range == ";" {
            return interpreter.handle_document_item(BqsqlDocumentItemType::Semicolon);
        }
    }

    None
}

impl BqsqlDocumentItem {
    pub(crate) fn new(
        item_type: BqsqlDocumentItemType,
        items: Vec<Option<BqsqlDocumentItem>>,
    ) -> BqsqlDocumentItem {
        let items = items.into_iter().filter_map(|f| f).collect();

        BqsqlDocumentItem {
            item_type: item_type,
            range: None,
            items: items,
        }
    }
}
