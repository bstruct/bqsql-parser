use crate::bqsql_document::*;
use lazy_static::lazy_static;
use regex::Regex;

impl BqsqlDocument {
    pub fn parse(bqsql: &str) -> BqsqlDocument {
        // print!("{}", bqsql);

        let position = BqsqlDocumentPosition::beginning_text();

        let mut items = Vec::new();

        if let Some(comment) = handle_comment(bqsql, position) {
            items.push(comment);
        }

        BqsqlDocument {
            document_type: BqsqlDocumentType::UNKNOWN,
            items: items,
        }
    }
}

fn handle_comment(bqsql: &str, position: BqsqlDocumentPosition) -> Option<BqsqlDocumentItem> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\s)?--.").unwrap();
    }
    if let Some(comment_match) = RE.find_at(bqsql, position.index) {
        // print!(comment_match.);
        let _pos2 = comment_match.range();
        let _pos1 = comment_match.start();

        return Some(BqsqlDocumentItem {
            item_type: BqsqlDocumentItemType::COMMENT,
            from: BqsqlDocumentPosition::beginning_text(),
            to: BqsqlDocumentPosition::beginning_text(),
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::bqsql_document::{BqsqlDocument, BqsqlDocumentItemType, BqsqlDocumentType};

    #[test]
    fn empty_string() {
        let document = BqsqlDocument::parse("");

        assert_eq!(BqsqlDocumentType::UNKNOWN, document.document_type);
        assert_eq!(0, document.items.len());
    }

    #[test]
    fn comment_only() {
        let document = BqsqlDocument::parse("--super comment");

        assert_eq!(BqsqlDocumentType::UNKNOWN, document.document_type);
        assert_eq!(1, document.items.len());
        assert_eq!(BqsqlDocumentItemType::COMMENT, document.items[0].item_type);
        assert_eq!(0, document.items[0].from.column);
        assert_eq!(0, document.items[0].from.line);
        assert_eq!(0, document.items[0].from.index);
        // assert_eq!(15, document.items[0].to.column);
        // assert_eq!(0, document.items[0].to.line);
        // assert_eq!(15, document.items[0].to.index);
    }

    #[test]
    fn space_comment_only() {
        let document = BqsqlDocument::parse("   --super comment");

        assert_eq!(BqsqlDocumentType::UNKNOWN, document.document_type);
        assert_eq!(1, document.items.len());
        assert_eq!(BqsqlDocumentItemType::COMMENT, document.items[0].item_type);
        // assert_eq!(4, document.items[0].from.column);
        // assert_eq!(1, document.items[0].from.line);
        // assert_eq!(4, document.items[0].from.index);
        // assert_eq!(19, document.items[0].to.column);
        // assert_eq!(1, document.items[0].to.line);
        // assert_eq!(19, document.items[0].to.index);
    }

    #[test]
    #[ignore]
    fn comment_and_tiny_query() {
        let document = BqsqlDocument::parse("--super comment\nSELECT 2+2");

        assert_eq!(BqsqlDocumentType::QUERY, document.document_type);
        assert_eq!(2, document.items.len());
        assert_eq!(BqsqlDocumentItemType::COMMENT, document.items[0].item_type);
        assert_eq!(0, document.items[0].from.column);
        assert_eq!(0, document.items[0].from.line);
        assert_eq!(0, document.items[0].from.index);
    }
}
