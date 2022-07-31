use crate::bqsql_document::token_parser;

use super::{bqsql_keyword::BqsqlKeyword, BqsqlDocumentItem, BqsqlDocumentItemType};

pub(crate) struct BqsqlInterpreter<'a> {
    //base elements that were parsed and need to be available for reference
    pub(crate) lines: Vec<&'a str>,
    pub(crate) tokens: Vec<[usize; 3]>,
    pub(crate) index: usize,
}

// pub(crate) struct BqsqlInterpreterDocumentItem<'a> {
//     pub(crate) interpreter: &'a BqsqlInterpreter<'a>,
//     pub(crate) document_item: Box<Vec<BqsqlDocumentItem>>,
// }

impl BqsqlInterpreter<'_> {
    pub(crate) fn new<'a>(bqsql: &'a str) -> BqsqlInterpreter {
        let lines = bqsql.lines().map(|l| l).collect::<Vec<&str>>();
        let tokens = token_parser::parse_tokens(bqsql);

        BqsqlInterpreter {
            lines: lines,
            tokens: tokens,
            index: 0,
        }
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
    // pub(crate) fn new_document_item<'a>(
    //     &'a self,
    //     document_item_type: BqsqlDocumentItemType,
    // ) -> BqsqlInterpreterDocumentItem {
    //     let document_item = BqsqlDocumentItem {
    //         item_type: document_item_type,
    //         range: None,
    //         items: vec![],
    //     };

    //     // BqsqlInterpreterDocumentItem {
    //     //     interpreter: self,
    //     //     document_item: Box::new(document_item),
    //     // }
    //     todo!()
    // }

    pub(crate) fn new_interpreter_items<'a>(&self) -> BqsqlInterpreterItems {
        BqsqlInterpreterItems {
            interpreter: self,
            items: Box::new(Vec::new()),
        }
    }
}

impl Iterator for BqsqlInterpreter<'_> {
    type Item = BqsqlDocumentItem;

    // next() is the only required method
    fn next(&mut self) -> Option<BqsqlDocumentItem> {
        let mut monitor_index = self.index;

        while self.tokens.len() > self.index {
            if let Some(query) = self.handle_query() {
                return Some(query);
            }

            if monitor_index == self.index {
                // self.handle_kunknown();
            } else {
                monitor_index = self.index;
            }

            self.index += 1;
        }

        None
    }
}

pub(crate) struct BqsqlInterpreterItems<'a> {
    //base elements that were parsed and need to be available for reference
    pub(crate) interpreter: &'a BqsqlInterpreter<'a>,
    items: Box<Vec<BqsqlDocumentItem>>,
}
impl<'a> BqsqlInterpreterItems<'a> {
    // pub(crate) fn append(&'a mut self, item: BqsqlDocumentItem) -> &'a BqsqlInterpreterItems<'a> {
    //     self.items.push(item);
    //     self
    // }
    // pub(crate) fn collect(&self)-> Vec<BqsqlDocumentItem>{
    //     self.items.to_vec()
    // }
    pub(crate) fn collect(&self, item_type: BqsqlDocumentItemType) -> BqsqlDocumentItem {
        BqsqlDocumentItem {
            item_type,
            range: None,
            items: self.items.to_vec(),
        }
    }
    pub(crate) fn collect_and_append(&'a mut self, item_type: BqsqlDocumentItemType) -> &'a BqsqlInterpreterItems<'a> {
        let item = BqsqlDocumentItem {
            item_type,
            range: None,
            items: self.items.to_vec(),
        };

        self.items.push(item);

        self
    }
    pub(crate) fn handle_keyword(
        &'a mut self,
        keyword: BqsqlKeyword,
    ) -> &'a mut BqsqlInterpreterItems<'a> {
        let interpreter = self.interpreter;
        if let Some(string_in_range) = interpreter.get_string_in_range(interpreter.index) {
            if string_in_range == keyword {
                self.items.push(BqsqlDocumentItem {
                    item_type: BqsqlDocumentItemType::Keyword,
                    range: Some(interpreter.tokens[interpreter.index]),
                    items: vec![],
                });
            }
        }
        self
    }
}

// impl BqsqlInterpreter {
//     // pub(crate) fn new<'a>(bqsql: &str) -> Box<BqsqlInterpreter> {
//     //     let lines = bqsql.lines().map(|l| l).collect::<Vec<&str>>();
//     //     let tokens = token_parser::parse_tokens(bqsql);

//     //     // let items = Vec::<Box<BqsqlDocumentItem>>::new();
//     //     let item = BqsqlDocumentItem {
//     //         item_type: BqsqlDocumentItemType::Unknown,
//     //         range: None,
//     //         items: vec![],
//     //     };

//     //     let interpreter = BqsqlInterpreter {
//     //         lines: Box::new(lines),
//     //         tokens: Box::new(tokens),
//     //         // lines: &lines,
//     //         // tokens: &tokens,
//     //         index: 0,
//     //         // top_node: Box::new(&top_node),
//     //         current_item: item,
//     //     };

//     //     Box::new(interpreter)
//     // }
//     // pub(crate) fn iterate<'a>(&'a mut self) -> &'a BqsqlInterpreter {
//     //     let mut monitor_index = self.index;

//     //     while self.tokens.len() > self.index {
//     //         self.handle_query();

//     //         if monitor_index == self.index {
//     //             self.handle_unknown();
//     //         } else {
//     //             monitor_index = self.index;
//     //         }

//     //         self.next();
//     //     }

//     //     self
//     // }
//     // pub(crate) fn next<'a>(&'a mut self) -> &'a BqsqlInterpreter {
//     //     self.index = self.index + 1;

//     //     self
//     // }
//     // pub(crate) fn handle_unknown(&mut self) -> &BqsqlInterpreter {
//     //     self.current_item.items.push(Box::new(BqsqlDocumentItem {
//     //         item_type: super::BqsqlDocumentItemType::Unknown,
//     //         range: Some(self.tokens[self.index]),
//     //         items: vec![],
//     //         parent: None,
//     //     }));

//     //     self.next()
//     // }

//     // /*
//     // / Add node and make it main node, so that when items are pushed via `append_node_item`,
//     // / these will be placed in the items of this new element
//     // */
//     // pub(crate) fn add_node_item<'a>(
//     //     &'a mut self,
//     //     item_type: BqsqlDocumentItemType,
//     // ) -> &'a BqsqlInterpreter {
//     //     let item = Box::new(BqsqlDocumentItem {
//     //         item_type: item_type,
//     //         range: None,
//     //         items: vec![],
//     //         parent: None, //Some(self.current_item),
//     //     });

//     //     self.current_item.items.push(item);

//     //     // self.current_node = Box::new(&item);

//     //     self
//     // }

//     // pub(crate) fn is_top_node(&self) -> bool {
//     //     // self.current_node.parent.is_none()

//     //     false
//     // }
//     // pub(crate) fn get_string_in_range(&self, index: usize) -> Option<&'_ str> {
//     //     if self.tokens.len() > index {
//     //         let range = &self.tokens[index];
//     //         if self.lines[range[0]].len() >= range[2] {
//     //             return Some(&self.lines[range[0]][range[1]..range[2]]);
//     //         }
//     //     }
//     //     None
//     // }
//     // pub(crate) fn get_bqsql_document(&self) -> BqsqlDocument {
//     //     BqsqlDocument { items: vec![] }
//     // }
// }
