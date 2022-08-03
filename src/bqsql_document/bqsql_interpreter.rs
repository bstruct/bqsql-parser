use crate::bqsql_document::token_parser;

use super::{bqsql_keyword::BqsqlKeyword, BqsqlDocumentItem, BqsqlDocumentItemType};

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
        if let Some(string_in_range) = self.get_string_in_range(self.index) {}
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

    pub(crate) fn collect(&mut self) -> Vec<BqsqlDocumentItem> {
        let mut monitor_index = self.index;
        let mut items = Vec::new();

        while self.tokens.len() > self.index {
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
