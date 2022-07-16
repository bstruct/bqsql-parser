use crate::bqsql_document::*;
use lazy_static::lazy_static;
use regex::Regex;

impl BqsqlDocument {
    pub fn parse(bqsql: &str) -> BqsqlDocument {
        // print!("{}", bqsql);

        lazy_static! {
            static ref RE: Regex = Regex::new(r"--.").unwrap();
        }

        let mut items = Vec::new();

        let comment = RE.find_at(bqsql, 0);
        if !comment.is_none() {
            items.push(BqsqlDocumentItem {
                item_type: BqsqlDocumentItemType::COMMENT,
                from: BqsqlDocumentPosition {
                    column: 0,
                    index: 0,
                    line: 0,
                },
                to: BqsqlDocumentPosition {
                    column: 0,
                    index: 0,
                    line: 0,
                },
            });
        }

        BqsqlDocument {
            document_type: BqsqlDocumentType::UNKNOWN,
            // items: String::from("qwer")
            items: items,
        }
    }
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
    }

    #[test]
    fn comment_and_tiny_query() {
        let document = BqsqlDocument::parse("--super comment\nSELECT 2+2");

        assert_eq!(BqsqlDocumentType::UNKNOWN, document.document_type);
        assert_eq!(2, document.items.len());
        assert_eq!(BqsqlDocumentItemType::COMMENT, document.items[0].item_type);
    }

}
