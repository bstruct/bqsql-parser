use crate::bqsql_document::*;
use lazy_static::lazy_static;
use regex::Regex;

impl BqsqlDocument {
    pub(crate) fn parse(bqsql: &str) -> BqsqlDocument {
        // let mut document_type = BqsqlDocumentType::UNKNOWN;
        // let mut position = BqsqlDocumentPosition::beginning_text();
        // let mut items = Vec::new();

        let tokens = parse_tokens(bqsql);

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

fn parse_tokens(bqsql: &str) -> Vec<BqsqlDocumentToken> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d?\.\d?|\b").unwrap();
        // static ref regex_comment: Regex = Regex::new("--.*").unwrap();

        //|\\d?\\.\\d?
        //|(\".+\")
        //(--.*$)|
    }

    let mut tokens: Vec<BqsqlDocumentToken> = Vec::new();

    let mut line_index: usize = 0;
    for line in bqsql.lines() {
        let mut gaps: Vec<[usize; 2]> = Vec::new();
        let mut previous_gap_start: usize = 0;

        for p1 in find_strings_and_line_comments(line) {
            gaps.push([previous_gap_start, p1[0]]);
            previous_gap_start = p1[1];

            let partial = line[p1[0]..p1[1]].to_string();
            tokens.push(BqsqlDocumentToken {
                from: BqsqlDocumentPosition::new(line_index, p1[0]),
                to: BqsqlDocumentPosition::new(line_index, p1[1]),
                token: partial,
            });
        }

        if gaps.len() == 0 {
            gaps.push([0, line.len()])
        };

        for gap in gaps {
            if gap[0] == gap[1] {
                continue;
            }

            let adjusted_line = &line[gap[0]..gap[1]];

            let split1 = RE
                .find_iter(adjusted_line)
                .map(|m| m.start())
                .collect::<Vec<usize>>();

            let mut previous_start: Option<usize> = None;

            // split1.push(adjusted_line.len());

            for start in split1 {
                if let Some(previous) = previous_start {
                    let partial = adjusted_line[previous..start].to_string();

                    // println!("partial: {}", partial);

                    if partial.trim().len() > 0 {
                        tokens.push(BqsqlDocumentToken {
                            from: BqsqlDocumentPosition::new(line_index, previous),
                            to: BqsqlDocumentPosition::new(line_index, start),
                            token: partial,
                        });
                    }
                }
                previous_start = Some(start);
            }
        }

        line_index = line_index + 1;
    }

    tokens.sort_by(|a, b| a.from.character.cmp(&b.from.character));

    tokens
}

#[test]
fn parse_tokens_single_line_operation() {
    let result = parse_tokens("    SELECT 23+2.45 --test, another `table` 123 \"back\" to 'dust'");

    assert_eq!(5, result.len());
    assert_eq!("SELECT", result[0].token);
    assert_eq!(0, result[0].from.line);
    assert_eq!(4, result[0].from.character);
    assert_eq!(0, result[0].to.line);
    assert_eq!(10, result[0].to.character);

    assert_eq!("23", result[1].token);
    assert_eq!(0, result[1].from.line);
    assert_eq!(11, result[1].from.character);
    assert_eq!(0, result[1].to.line);
    assert_eq!(13, result[1].to.character);

    assert_eq!("+", result[2].token);
    assert_eq!(0, result[2].from.line);
    assert_eq!(13, result[2].from.character);
    assert_eq!(0, result[2].to.line);
    assert_eq!(14, result[2].to.character);

    assert_eq!("2.45", result[3].token);
    assert_eq!(0, result[3].from.line);
    assert_eq!(14, result[3].from.character);
    assert_eq!(0, result[3].to.line);
    assert_eq!(18, result[3].to.character);

    assert_eq!(
        "--test, another `table` 123 \"back\" to 'dust'",
        result[4].token
    );
    assert_eq!(0, result[4].from.line);
    assert_eq!(19, result[4].from.character);
    assert_eq!(0, result[4].to.line);
    assert_eq!(63, result[4].to.character);
}

#[test]
fn parse_tokens_single_line_string() {
    let result = parse_tokens(
        "SELECT \"this is a ''' string \" --test, another `table` 123 \"back\" to 'dust'",
    );

    assert_eq!(3, result.len());
    assert_eq!("SELECT", result[0].token);
    assert_eq!(0, result[0].from.line);
    assert_eq!(0, result[0].from.character);
    assert_eq!(0, result[0].to.line);
    assert_eq!(6, result[0].to.character);

    assert_eq!("\"this is a ''' string \"", result[1].token);
    assert_eq!(0, result[1].from.line);
    assert_eq!(7, result[1].from.character);
    assert_eq!(0, result[1].to.line);
    assert_eq!(30, result[1].to.character);

    assert_eq!(
        "--test, another `table` 123 \"back\" to 'dust'",
        result[2].token
    );
    assert_eq!(0, result[2].from.line);
    assert_eq!(31, result[2].from.character);
    assert_eq!(0, result[2].to.line);
    assert_eq!(75, result[2].to.character);
}

#[test]
fn parse_tokens_single_line_string_with_double_dash() {
    let result = parse_tokens("SELECT \"this is a -- string \"  ");

    assert_eq!(2, result.len());
    assert_eq!("SELECT", result[0].token);
    assert_eq!(0, result[0].from.line);
    assert_eq!(0, result[0].from.character);
    assert_eq!(0, result[0].to.line);
    assert_eq!(6, result[0].to.character);

    assert_eq!("\"this is a -- string \"", result[1].token);
    assert_eq!(0, result[1].from.line);
    assert_eq!(7, result[1].from.character);
    assert_eq!(0, result[1].to.line);
    assert_eq!(29, result[1].to.character);
}

fn find_strings_and_line_comments(line: &str) -> Vec<[usize; 2]> {
    let mut possible_escape_char = false;
    let mut possible_line_comment = false;

    let mut positions: Vec<[usize; 2]> = Vec::new();
    let mut index: usize = 0;
    let mut previous_double_quote: Option<usize> = None;
    let mut previous_single_quote: Option<usize> = None;

    for character in line.chars() {
        if character == '\\' {
            possible_escape_char = true;
            index = index + 1;
            continue;
        }
        if character == '-' && previous_double_quote.is_none() && previous_single_quote.is_none() {
            if possible_line_comment {
                positions.push([index - 1, line.len()]);
                return positions;
            }
            possible_line_comment = true;
            index = index + 1;
            continue;
        }

        if character == '"' && previous_single_quote.is_none() {
            if !possible_escape_char {
                if previous_double_quote.is_some() {
                    positions.push([previous_double_quote.unwrap(), index + 1]);
                    previous_double_quote = None;
                } else {
                    previous_double_quote = Some(index);
                }
            }
        }

        if character == '\'' && previous_double_quote.is_none() {
            if !possible_escape_char {
                if previous_single_quote.is_some() {
                    positions.push([previous_single_quote.unwrap(), index + 1]);
                    previous_single_quote = None;
                } else {
                    previous_single_quote = Some(index);
                }
            }
        }

        possible_escape_char = false;
        index = index + 1;
    }

    positions
}

#[test]
fn find_strings_and_line_comments_no_string_no_comment() {
    let result = find_strings_and_line_comments(" SELECT 23-2.45");

    assert_eq!(0, result.len());
}

#[test]
fn find_strings_and_line_comments_no_string_with_comment() {
    let result = find_strings_and_line_comments(
        " SELECT 23+2.45 --test, another `table` 123 \"back\" to 'dust'",
    );

    assert_eq!(1, result.len());
    assert_eq!(16, result[0][0]);
    assert_eq!(60, result[0][1]);
}

#[test]
fn find_strings_and_line_comments_double_quote_string_and_comment() {
    let result = find_strings_and_line_comments(
        " SELECT \"this is a \\\" -- string \" --test, another `table` 123 \"back\" to 'dust'",
    );

    assert_eq!(2, result.len());
    assert_eq!(8, result[0][0]);
    assert_eq!(33, result[0][1]);

    assert_eq!(34, result[1][0]);
    assert_eq!(78, result[1][1]);
}

#[test]
fn find_strings_and_line_comments_single_quote_string_and_comment() {
    let result = find_strings_and_line_comments(
        " SELECT 'this is a \\' -- string ' --test, another `table` 123 \"back\" to 'dust'",
    );

    assert_eq!(2, result.len());
    assert_eq!(8, result[0][0]);
    assert_eq!(33, result[0][1]);

    assert_eq!(34, result[1][0]);
    assert_eq!(78, result[1][1]);
}

#[test]
fn find_strings_and_line_comments_strings() {
    let result = find_strings_and_line_comments(
        " SELECT 'this is a \\' -- string ',\"this is also a \\\" -- string \"",
    );

    assert_eq!(4, result.len());
    // assert_eq!(8, result[0][0]);
    // assert_eq!(33, result[0][1]);

    // assert_eq!(34, result[1][0]);
    // assert_eq!(78, result[1][1]);
}

// fn handle_comment(bqsql: &str, position: &BqsqlDocumentPosition) -> Option<BqsqlDocumentItem> {
//     lazy_static! {
//         static ref RE: Regex = Regex::new(r"^\s*--.*").unwrap();
//     }
//     if let Some(comment_match) = RE.find_at(bqsql, position.index) {
//         let end = comment_match.end();

//         return Some(BqsqlDocumentItem {
//             item_type: BqsqlDocumentItemType::COMMENT,
//             from: BqsqlDocumentPosition::move_to_non_white(bqsql, position),
//             to: BqsqlDocumentPosition::move_end(position, end - 1),
//         });
//     }
//     None
// }

// fn handle_query(bqsql: &str, position: &BqsqlDocumentPosition) -> Option<BqsqlDocumentItem> {
//     lazy_static! {
//         static ref RE: Regex = Regex::new(r"^[\t\v\f\r ]*SELECT\s+").unwrap();
//     }

//     if let Some(select_match) = RE.find(&bqsql[position.index..]) {
//         let end = select_match.end();

//         //find the position of the word "SELECT"

//         //const or simple calculation as type, example: "hi" AS name, or 1 AS number, or 2+2 as another_number
//         //column with potential alias

//         return Some(BqsqlDocumentItem {
//             item_type: BqsqlDocumentItemType::QUERY,
//             from: BqsqlDocumentPosition::move_to_non_white(bqsql, position),
//             to: BqsqlDocumentPosition::move_end(position, end),
//         });
//     }
//     None
// }

// #[test]
// fn test_handle_query() {
//     let result = handle_query("SELECT 2+2", &BqsqlDocumentPosition::beginning_text());
//     assert!(result.is_some());
// }

// #[test]
// fn test_handle_query_after_comment() {
//     let result = handle_query(
//         "--super comment\nSELECT 2+2",
//         &BqsqlDocumentPosition {
//             column: 0,
//             line: 1,
//             index: 16,
//         },
//     );
//     assert!(result.is_some());
// }

// #[test]
// fn test_handle_query_new_line() {
//     let result = handle_query("\nSELECT 2+2", &BqsqlDocumentPosition::beginning_text());
//     assert!(result.is_none());
// }

// #[test]
// fn test_handle_query_new_line_space() {
//     let result = handle_query("\n SELECT 2+2", &BqsqlDocumentPosition::beginning_text());
//     assert!(result.is_none());
// }

// #[cfg(test)]
// mod test_select;
