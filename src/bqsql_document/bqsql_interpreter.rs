use crate::bqsql_document::token_parser;

use super::{BqsqlDocumentItem, BqsqlDocumentItemType};

pub(crate) struct BqsqlInterpreter<'a> {
    //base elements that were parsed and need to be available for reference
    pub(crate) lines: Box<Vec<&'a str>>,
    pub(crate) tokens: Box<Vec<[usize; 3]>>,

    //tree navigation control
    pub(crate) index: usize,
    pub(crate) current_node: Box<Option<BqsqlDocumentItem>>,

    //generated elements
    pub(crate) items: Box<Vec<Box<BqsqlDocumentItem>>>,
}

impl BqsqlInterpreter<'_> {
    pub(crate) fn new(bqsql: &str) -> BqsqlInterpreter {
        let lines = bqsql.lines().map(|l| l).collect::<Vec<&str>>();
        let tokens = token_parser::parse_tokens(bqsql);
        let items = Box::new(Vec::<Box<BqsqlDocumentItem>>::new());

        BqsqlInterpreter {
            lines: Box::new(lines),
            tokens: Box::new(tokens),
            index: 0,
            current_node: Box::new(None),
            items,
        }
    }
    pub(crate) fn iterate(&mut self) -> &BqsqlInterpreter {
        let mut monitor_index = self.index;

        while self.tokens.len() > self.index {
            self.handle_query();

            if monitor_index == self.index {
                self.handle_unknown();
            } else {
                monitor_index = self.index;
            }

            self.next();
        }

        self
    }
    pub(crate) fn next(&mut self) -> &BqsqlInterpreter {
        self.index = self.index + 1;

        self
    }
    pub(crate) fn handle_unknown(&mut self) -> &BqsqlInterpreter {
        // self.items.push(BqsqlDocumentItem {
        //     item_type: super::BqsqlDocumentItemType::Unknown,
        //     range: Some(self.tokens[self.index]),
        //     items: vec![],
        // });

        self.next()
    }

    pub(crate) fn add_node_item(&mut self, item_type: BqsqlDocumentItemType) -> &BqsqlInterpreter {
        if self.current_node.is_some() {

            let item = BqsqlDocumentItem {
                item_type: item_type,
                range: None,
                items: Box::new(vec![]),
                parent: self.current_node,
            };
            
            self.current_node.unwrap().items.push(item);
            
            self.current_node = Box::new(Some(item));
            
            todo!()

        } else {
            let item = BqsqlDocumentItem {
                item_type: item_type,
                range: None,
                items: Box::new(vec![]),
                parent: Box::new(None),
            };

            self.current_node = Box::new(Some(item));

            self.items.push(Box::new(item));
        }

        self
    }

    pub(crate) fn is_top_node(&self) -> bool {
        self.current_node.is_none()
    }
    pub(crate) fn get_string_in_range(&self, index: usize) -> Option<&'_ str> {
        if self.tokens.len() > index {
            let range = &self.tokens[index];
            if self.lines[range[0]].len() >= range[2] {
                return Some(&self.lines[range[0]][range[1]..range[2]]);
            }
        }
        None
    }
    pub(crate) fn get_bqsql_document(&self) -> Vec<Box<BqsqlDocumentItem>> {
        self.items.to_vec()
    }
}
