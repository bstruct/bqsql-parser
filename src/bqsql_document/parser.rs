use crate::bqsql_document::*;

use crate::bqsql_document::token_parser;

impl BqsqlDocument {
    pub(crate) fn parse(bqsql: &str) -> BqsqlDocument {
        // let mut document_type = BqsqlDocumentType::UNKNOWN;
        // let mut position = BqsqlDocumentPosition::beginning_text();
        let mut items: Vec<BqsqlDocumentItem> = Vec::new();

        let lines = &bqsql.lines().map(|l| l).collect::<Vec<&str>>();
        let tokens = &token_parser::parse_tokens(bqsql);

        let mut index: usize = 0;

        while index < tokens.len() {
            //comments are not relevant for now
            if is_comment(lines, tokens, index) {
                index = index + 1;
                continue;
            }

            if let Some((query, new_index)) = handle_query(lines, tokens, index) {
                index = new_index;
                items.push(query);
                continue;
            }

            //for now, only queries are supported.
            //if there's other types of syntaxes, (DDL, DML, DCL,...), they will be ignored
            while get_string_in_range(lines, &tokens[index]) != ";" || index >= tokens.len() {
                index = index + 1;
            }

            index = index + 1;
        }

        BqsqlDocument { items: items }
    }
}

fn is_comment<'a>(lines: &Vec<&'a str>, tokens: &[[usize; 3]], index: usize) -> bool {
    get_string_in_range(lines, &tokens[index]).starts_with("--")
}

fn handle_query<'a>(lines: &Vec<&'a str>, tokens: &[[usize; 3]], index: usize) -> Option<(BqsqlDocumentItem, usize)> {

    let string_in_range = get_string_in_range(lines, &tokens[index]).to_uppercase();

    if string_in_range == "SELECT" || string_in_range == "WITH" {
        let item = BqsqlDocumentItem {
            item_type: BqsqlDocumentItemType::QUERY,
            range: None,
            items: vec![
                // BqsqlDocumentItem { item_type: BqsqlDocumentItemType::QUERY_SELECT_KEYWORD, range: &tokens[index].range, items: vec![] }
            ],
        };

        return Some((item, index + 1));
    }

    None
}

fn get_string_in_range<'a>(lines: &Vec<&'a str>, range: &[usize; 3]) -> &'a str {
    &lines[range[0]][range[1]..range[2]]
}
