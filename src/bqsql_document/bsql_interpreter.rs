use crate::bqsql_document::token_parser;

use super::{BqsqlDocument, BqsqlDocumentItem};

pub(crate) struct BqsqlInterpreter<'a> {
    lines: Vec<&'a str>,
    pub items: Box<Vec<BqsqlDocumentItem>>,
}

impl BqsqlInterpreter<'_> {
    pub(crate) fn new(bqsql: &str) -> BqsqlInterpreter {
        let lines = bqsql.lines().map(|l| l).collect::<Vec<&str>>();
        let tokens = &token_parser::parse_tokens(bqsql);
        let items= Box::new(Vec::<BqsqlDocumentItem>::new());

        BqsqlInterpreter {
            lines: lines,
            items: items,
        }

        // todo!();
    }
    pub(crate) fn iterate(&self) -> BqsqlInterpreter {
        todo!();
    }
    pub(crate) fn compile(&self) -> BqsqlDocument {
        BqsqlDocument {
            items: self.items.to_vec(),
        }
    }
}
