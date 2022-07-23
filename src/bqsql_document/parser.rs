use crate::bqsql_document::*;

use crate::bqsql_document::token_parser;

impl BqsqlDocument {
    pub(crate) fn parse(bqsql: &str) -> BqsqlDocument {
        // let mut document_type = BqsqlDocumentType::UNKNOWN;
        // let mut position = BqsqlDocumentPosition::beginning_text();
        // let mut items = Vec::new();

        let tokens = token_parser::parse_tokens(bqsql);

        // while position.index < bqsql.len() {
        //     let mut skip = false;

        //     if let Some(comment) = handle_comment(bqsql, &position) {
        //         position = BqsqlDocumentPosition::copy(&comment.to);
        //         items.push(comment);
        //         skip = true;
        //     }

        //     if !skip {
        //         if let Some(query) = handle_query(bqsql, &position) {
        //             document_type = BqsqlDocumentType::QUERY;
        //             position = BqsqlDocumentPosition::copy(&query.to);
        //             items.push(query);
        //         }
        //     }

        //     if let Some(next_position) = BqsqlDocumentPosition::next(bqsql, &position) {
        //         position = next_position;
        //     } else {
        //         break;
        //     }
        // }

        BqsqlDocument {
            document_type: BqsqlDocumentType::UNKNOWN,
            items: tokens,
        }
    }
}
