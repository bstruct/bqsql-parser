use lazy_static::lazy_static;
use regex::Regex;

use super::{
    bqsql_delimiter::BqsqlDelimiter, bqsql_keyword::BqsqlKeyword, bqsql_operator::BqsqlOperator,
    BqsqlDocumentItem, BqsqlDocumentItemType,
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

    pub(crate) fn is_keyword(&self, keyword: BqsqlKeyword) -> bool {
        if let Some(string_in_range) = self.get_string_in_range(self.index) {
            return string_in_range == keyword;
        }
        false
    }

    pub(crate) fn is_line_comment(&self) -> bool {
        if let Some(string_in_range) = self.get_string_in_range(self.index) {
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

    pub(crate) fn handle_keyword(&mut self, keyword: BqsqlKeyword) -> Option<BqsqlDocumentItem> {
        self.handle_string(BqsqlDocumentItemType::Keyword, keyword.as_str())
    }

    pub(crate) fn handle_string(
        &mut self,
        item_type: BqsqlDocumentItemType,
        comp: &str,
    ) -> Option<BqsqlDocumentItem> {
        if let Some(string_in_range) = self.get_string_in_range(self.index) {
            if string_in_range == comp {
                return self.handle_document_item(item_type);
            }
        }
        None
    }

    pub(crate) fn handle_kunknown(&mut self) -> BqsqlDocumentItem {
        let item = BqsqlDocumentItem {
            item_type: BqsqlDocumentItemType::Unknown,
            range: Some(self.tokens[self.index]),
            items: vec![],
        };

        self.index += 1;

        item
    }

    pub(crate) fn is_in_range(&self, index: usize) -> bool {
        self.tokens.len() > index
    }

    pub(crate) fn is_number(&self) -> bool {
        if let Some(string_in_range) = self.get_string_in_range(self.index) {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"\d+|\d*\.{1}\d*").unwrap();
            }

            return RE.is_match(string_in_range);
        }
        false
    }

    pub(crate) fn is_delimiter(&self, index: usize, delimiter: BqsqlDelimiter) -> bool {
        if let Some(string_in_range) = self.get_string_in_range(index) {
            return string_in_range == delimiter;
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

    pub(crate) fn find_any_operator(&self, index: usize) -> Option<BqsqlOperator> {
        let all = &BqsqlOperator::get_all();

        if self.is_in_range(index + 2) {
            let values3 = vec![
                self.get_string_in_range(index),
                self.get_string_in_range(index + 1),
                self.get_string_in_range(index + 2),
            ]
            .iter()
            .map(|i| i.unwrap().to_string())
            .collect::<Vec<String>>();
            let mut op3 = all
                .iter()
                .map(|i| (i, i.to_vec()))
                .filter(|i| i.1.len() == 3);

            if let Some(m) = op3.find(|i| i.1.to_vec() == values3) {
                return Some(m.0.to_owned());
            }
        }

        if self.is_in_range(index + 1) {
            let values2 = vec![
                self.get_string_in_range(index),
                self.get_string_in_range(index + 1),
            ]
            .iter()
            .map(|i| i.unwrap().to_string())
            .collect::<Vec<String>>();
            let mut op2 = all
                .iter()
                .map(|i| (i, i.to_vec()))
                .filter(|i| i.1.len() == 2);

            if let Some(m) = op2.find(|i| i.1.to_vec() == values2) {
                return Some(m.0.to_owned());
            }
        }

        if self.is_in_range(index) {
            let values1 = vec![self.get_string_in_range(index)]
                .iter()
                .map(|i| i.unwrap().to_string())
                .collect::<Vec<String>>();
            let mut op1 = all
                .iter()
                .map(|i| (i, i.to_vec()))
                .filter(|i| i.1.len() == 1);

            if let Some(m) = op1.find(|i| i.1.to_vec() == values1) {
                return Some(m.0.to_owned());
            }
        }

        None
    }

    pub(crate) fn collect(&mut self) -> Vec<BqsqlDocumentItem> {
        let mut monitor_index = self.index;
        let mut items: Vec<BqsqlDocumentItem> = Vec::new();

        while self.tokens.len() > self.index {
            if self.is_line_comment() {
                items.push(self.handle_document_item(BqsqlDocumentItemType::LineComment).unwrap());
            }
            
            if let Some(query) = self.handle_query() {
                items.push(query);
            }

            if monitor_index == self.index {
                items.push(self.handle_kunknown());
            } else {
                monitor_index = self.index;
            }

            self.index += 1;
        }

        items
    }
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
