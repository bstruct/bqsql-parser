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

    pub(crate) fn handle_keyword(&mut self, keyword: BqsqlKeyword) -> Option<BqsqlDocumentItem> {
        if let Some(string_in_range) = self.get_string_in_range(self.index) {
            if string_in_range == keyword {
                let item = BqsqlDocumentItem {
                    item_type: BqsqlDocumentItemType::Keyword,
                    range: Some(self.tokens[self.index]),
                    items: vec![],
                };

                self.index += 1;

                return Some(item);
            }
        }
        None
    }

    pub(crate) fn handle_string(
        &mut self,
        item_type: BqsqlDocumentItemType,
        comp: &str,
    ) -> Option<BqsqlDocumentItem> {
        if let Some(string_in_range) = self.get_string_in_range(self.index) {
            if string_in_range == comp {
                let item = BqsqlDocumentItem {
                    item_type: item_type,
                    range: Some(self.tokens[self.index]),
                    items: vec![],
                };

                self.index += 1;

                return Some(item);
            }
        }
        None
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

    pub(crate) fn collect(&mut self) -> Vec<BqsqlDocumentItem> {
        let mut monitor_index = self.index;
        let mut items = Vec::new();

        while self.tokens.len() > self.index {
            if let Some(query) = self.handle_query() {
                items.push(query);
            }

            if monitor_index == self.index {
                // self.handle_kunknown();
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

// impl<'a> Iterator for BqsqlInterpreter<'a> {
//     type Item = BqsqlDocumentItem;
//     // next() is the only required method
//     fn next(&'a mut self) -> Option<BqsqlDocumentItem> {
//         let mut monitor_index = self.index;
//         while self.tokens.len() > self.index {
//             if let Some(query) = self.handle_query() {
//                 return Some(query);
//             }
//             if monitor_index == self.index {
//                 // self.handle_kunknown();
//             } else {
//                 monitor_index = self.index;
//             }
//             self.index += 1;
//         }
//         None
//     }
// }

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
