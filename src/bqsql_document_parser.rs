use crate::bqsql_document::*;
use lazy_static::lazy_static;
use regex::Regex;

impl BqsqlDocument {
    pub(crate) fn parse(bqsql: &str) -> BqsqlDocument {
        let mut document_type = BqsqlDocumentType::UNKNOWN;
        let mut position = BqsqlDocumentPosition::beginning_text();
        let mut items = Vec::new();

        while position.index < bqsql.len() {
            let mut skip = false;

            if let Some(comment) = handle_comment(bqsql, &position) {
                position = BqsqlDocumentPosition::copy(&comment.to);
                items.push(comment);
                skip = true;
            }

            if !skip {
                if let Some(query) = handle_query(bqsql, &position) {
                    document_type = BqsqlDocumentType::QUERY;
                    position = BqsqlDocumentPosition::copy(&query.to);
                    items.push(query);
                }
            }

            if let Some(next_position) = BqsqlDocumentPosition::next(bqsql, &position) {
                position = next_position;
            } else {
                break;
            }
        }

        BqsqlDocument {
            document_type: document_type,
            items: items,
        }
    }
}

fn parse_tokens(bqsql: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(--.*)|\b").unwrap();
    }

    // let mut line: usize = 0;

    let mut split1 = RE.find_iter(bqsql).map(|m| m.start())
    .collect::<Vec<usize>>();

    let mut previous_start: Option<usize> = None;

    // let v = Vec::from(..split1);
    split1.push(bqsql.len());

    for start in split1 {
        if let Some(previous) = previous_start {
            let partial = bqsql[previous..start].to_string();
            println!("{}", partial);
        }
        previous_start = Some(start);
    }
    // for m in split1 {
    //     // for c1 in c.iter() {
    //     //     if let Some(m) = c1 {

    //     let b = m.start();
    //     let e = m.end();
    //     let s = m.as_str().to_string();

    // println!("{}", split1.join(","));
    //     //     }
    //     // }
    // }

    // let _split1 = bqsql
    //     .lines()
    //     .map(|l| RE.split(l))
    //     .flatten()
    //     .map(|s| s.to_string())
    //     .collect::<Vec<String>>();

    // let split1 = RE.split(bqsql)
    //     .map(|l| l.to_string())
    //     .filter(|l| l.trim().len() > 0)
    //     .collect::<Vec<String>>();

    // let split = RE.split(bqsql);
    // print!("{}", split1.len());

    Vec::new()
}

#[test]
fn parse_tokens_1() {
    let result = parse_tokens("    SELECT 23+2 --test, another `table` 123 \"back\" to 'dust'");
    assert_eq!(0, result.len());
    // assert_eq!(4, result.len());
    // assert_eq!("SELECT", result[0]);
    // assert_eq!("23", result[1]);
    // assert_eq!("+", result[2]);
    // assert_eq!("2", result[3]);
}

fn handle_comment(bqsql: &str, position: &BqsqlDocumentPosition) -> Option<BqsqlDocumentItem> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*--.*").unwrap();
    }
    if let Some(comment_match) = RE.find_at(bqsql, position.index) {
        let end = comment_match.end();

        return Some(BqsqlDocumentItem {
            item_type: BqsqlDocumentItemType::COMMENT,
            from: BqsqlDocumentPosition::move_to_non_white(bqsql, position),
            to: BqsqlDocumentPosition::move_end(position, end - 1),
        });
    }
    None
}

fn handle_query(bqsql: &str, position: &BqsqlDocumentPosition) -> Option<BqsqlDocumentItem> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[\t\v\f\r ]*SELECT\s+").unwrap();
    }

    if let Some(select_match) = RE.find(&bqsql[position.index..]) {
        let end = select_match.end();

        //find the position of the word "SELECT"

        //const or simple calculation as type, example: "hi" AS name, or 1 AS number, or 2+2 as another_number
        //column with potential alias

        return Some(BqsqlDocumentItem {
            item_type: BqsqlDocumentItemType::QUERY,
            from: BqsqlDocumentPosition::move_to_non_white(bqsql, position),
            to: BqsqlDocumentPosition::move_end(position, end),
        });
    }
    None
}

#[test]
fn test_handle_query() {
    let result = handle_query("SELECT 2+2", &BqsqlDocumentPosition::beginning_text());
    assert!(result.is_some());
}

#[test]
fn test_handle_query_after_comment() {
    let result = handle_query(
        "--super comment\nSELECT 2+2",
        &BqsqlDocumentPosition {
            column: 0,
            line: 1,
            index: 16,
        },
    );
    assert!(result.is_some());
}

#[test]
fn test_handle_query_new_line() {
    let result = handle_query("\nSELECT 2+2", &BqsqlDocumentPosition::beginning_text());
    assert!(result.is_none());
}

#[test]
fn test_handle_query_new_line_space() {
    let result = handle_query("\n SELECT 2+2", &BqsqlDocumentPosition::beginning_text());
    assert!(result.is_none());
}

#[cfg(test)]
mod test_select;
