use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_tokens(bqsql: &str) -> Vec<[usize; 3]> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"`.*`|'.*'|\d*\.{1}\d*|[A-z0-9_]+|\W?").unwrap();
    }

    let mut tokens: Vec<[usize; 3]> = Vec::new();

    let mut line_index: usize = 0;
    for line in bqsql.lines() {
        let mut gaps: Vec<[usize; 2]> = Vec::new();
        let mut previous_gap_start: usize = 0;

        for p1 in find_strings_and_line_comments(line) {
            gaps.push([previous_gap_start, p1[0]]);
            previous_gap_start = p1[1];

            // let partial = line[p1[0]..p1[1]].to_string();
            // tokens.push(BqsqlDocumentToken {
            //     from: BqsqlDocumentPosition::new(line_index, p1[0]),
            //     to: BqsqlDocumentPosition::new(line_index, p1[1]),
            //     token: partial,
            // });
            tokens.push([line_index, p1[0], p1[1]]);
        }

        if gaps.len() == 0 {
            gaps.push([0, line.len()])
        };

        for gap in gaps {
            if gap[0] == gap[1] {
                continue;
            }

            let adjusted_line = &line[gap[0]..gap[1]];

            for m in RE.find_iter(adjusted_line) {
                let partial = &adjusted_line[m.start()..m.end()];
                if partial.trim().len() > 0 {
                    // tokens.push(BqsqlDocumentToken {
                    //     from: BqsqlDocumentPosition::new(line_index, gap[0] + m.start()),
                    //     to: BqsqlDocumentPosition::new(line_index, gap[0] + m.end()),
                    //     token: partial,
                    // });
                    tokens.push([line_index, gap[0] + m.start(), gap[0] + m.end()]);
                }
            }
        }

        line_index = line_index + 1;
    }

    tokens.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

    tokens
}

#[test]
fn parse_tokens_single_line_operation() {
    let result = parse_tokens("    SELECT 23+2.45 --test, another `table` 123 \"back\" to 'dust'");

    assert_eq!(5, result.len());
    assert_eq!([0, 4, 10], result[0]);
    assert_eq!([0, 11,13], result[1]);
    assert_eq!([0, 13, 14], result[2]);
    assert_eq!([0, 14, 18], result[3]);
    assert_eq!([0, 19, 63], result[4]);

}

#[test]
fn parse_tokens_single_line_string() {
    let result = parse_tokens(
        "SELECT \"this is a ''' string \" --test, another `table` 123 \"back\" to 'dust'",
    );

    assert_eq!(3, result.len());

    assert_eq!([0, 0, 6], result[0]);
    assert_eq!([0, 7, 30], result[1]);
    assert_eq!([0, 31, 75], result[2]);

}

#[test]
fn parse_tokens_parenthisis() {
    let result = parse_tokens("SELECT (((1)))");

    assert_eq!(8, result.len());

    assert_eq!([0, 0, 6], result[0]);
    assert_eq!([0, 7, 8], result[1]);
    assert_eq!([0, 8, 9], result[2]);
    assert_eq!([0, 9, 10], result[3]);
    assert_eq!([0, 10, 11], result[4]);
    assert_eq!([0, 11, 12], result[5]);
    assert_eq!([0, 12, 13], result[6]);
    assert_eq!([0, 13, 14], result[7]);

}

#[test]
fn parse_tokens_single_line_string_with_double_dash() {
    let result = parse_tokens("SELECT \"this is a -- string \"  ");

    assert_eq!(2, result.len());

    assert_eq!([0, 0, 6], result[0]);
    assert_eq!([0, 7, 29], result[1]);

}

#[test]
#[ignore = "the single quote string parse was moved to the `parse_tokens` function"]
fn parse_tokens_strings() {
    let result =
        parse_tokens(" SELECT 'this is a \\' -- string ',\"this is also a \\\" -- string \"");

    assert_eq!(4, result.len());

    assert_eq!([0, 1, 7], result[0]);
    assert_eq!([0, 8, 33], result[1]);
    assert_eq!([0, 33, 34], result[2]);
    assert_eq!([0, 34, 64], result[3]);

}

#[test]
#[ignore = "the single quote string parse was moved to the `parse_tokens` function"]
fn parse_tokens_strings_with_space() {
    let result =
        parse_tokens(" SELECT 'this is a \\' -- string ', \"this is also a \\\" -- string \"");

    assert_eq!(4, result.len());

    assert_eq!([0, 1, 7], result[0]);
    assert_eq!([0, 8, 33], result[1]);
    assert_eq!([0, 33, 34], result[2]);
    assert_eq!([0, 35, 65], result[3]);

}

#[test]
fn parse_tokens_strings_multi_select() {
    let result =
        parse_tokens(" SELECT (SELECT AS STRUCT 2+2 AS asas, 'ASDASD' AS qweqwe) AS XXX");

    assert_eq!(17, result.len());

    assert_eq!([0, 1, 7], result[0]);

}

#[test]
fn parse_tokens_single_quote_in_string() {
    let result =
        parse_tokens("SELECT 'Timmy O\'Hara'");

    assert_eq!(2, result.len());

    assert_eq!([0, 0, 6], result[0]);
    assert_eq!([0, 7, 21], result[1]);

}

#[test]
fn parse_tokens_double_quote_in_string() {
    let result =
        parse_tokens("SELECT \"Timmy O'Hara\"");

    assert_eq!(2, result.len());

    assert_eq!([0, 0, 6], result[0]);
    assert_eq!([0, 7, 21], result[1]);

}

#[test]
fn parse_tokens_single_quote_in_string_double_escape() {
    let result =
        parse_tokens("SELECT 'Timmy O\\\'Hara'");

    assert_eq!(2, result.len());

    assert_eq!([0, 0, 6], result[0]);
    assert_eq!([0, 7, 22], result[1]);

}

#[test]
fn parse_tokens_single_quote_in_string_double_escape_multiple_columns() {
    let result =
        parse_tokens("SELECT 'Timmy O\\\'Hara', 2 AS second_column");

    assert_eq!(6, result.len());

    assert_eq!([0, 0, 6], result[0]);
    assert_eq!([0, 7, 22], result[1]);
    assert_eq!([0, 22, 23], result[2]);
    assert_eq!([0, 24, 25], result[3]);
    assert_eq!([0, 26, 28], result[4]);
    assert_eq!([0, 29, 42], result[5]);

}

#[test]
fn parse_tokens_string_hell() {
    let result =
        parse_tokens("    SELECT 23+2.45, \n'another\"\"\"\"\"\"\" \\\' test', \"test ''''''' \\\" this\"");

    assert_eq!(8, result.len());

    // assert_eq!([0, 0, 6], result[0]);
    // assert_eq!([0, 7, 22], result[1]);
    // assert_eq!([0, 22, 23], result[2]);
    // assert_eq!([0, 24, 25], result[3]);
    // assert_eq!([0, 26, 28], result[4]);
    // assert_eq!([0, 29, 42], result[5]);

}

fn find_strings_and_line_comments(line: &str) -> Vec<[usize; 2]> {
    let mut possible_escape_char = false;
    let mut possible_line_comment = false;

    let mut positions: Vec<[usize; 2]> = Vec::new();
    let mut index: usize = 0;
    let mut previous_double_quote: Option<usize> = None;
    // let mut previous_single_quote: Option<usize> = None;

    for character in line.chars() {
        if character == '\\' {
            possible_escape_char = true;
            index = index + 1;
            continue;
        }
        if character == '-' && previous_double_quote.is_none() //&& previous_single_quote.is_none() 
        {
            if possible_line_comment {
                positions.push([index - 1, line.len()]);
                return positions;
            }
            possible_line_comment = true;
            index = index + 1;
            continue;
        }

        if character == '"' //&& previous_single_quote.is_none() 
        {
            if !possible_escape_char {
                if previous_double_quote.is_some() {
                    positions.push([previous_double_quote.unwrap(), index + 1]);
                    previous_double_quote = None;
                } else {
                    previous_double_quote = Some(index);
                }
            }
        }

        // if character == '\'' && previous_double_quote.is_none() {
        //     if !possible_escape_char {
        //         if previous_single_quote.is_some() {
        //             positions.push([previous_single_quote.unwrap(), index + 1]);
        //             previous_single_quote = None;
        //         } else {
        //             previous_single_quote = Some(index);
        //         }
        //     }
        // }

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
    assert_eq!([16, 60], result[0]);
}

#[test]
fn find_strings_and_line_comments_double_quote_string_and_comment() {
    let result = find_strings_and_line_comments(
        " SELECT \"this is a \\\" -- string \" --test, another `table` 123 \"back\" to 'dust'",
    );

    assert_eq!(2, result.len());
    assert_eq!([8, 33], result[0]);
    assert_eq!([34, 78], result[1]);
}

#[test]
#[ignore = "the single quote string parse was moved to the `parse_tokens` function"]
fn find_strings_and_line_comments_single_quote_string_and_comment() {
    let result = find_strings_and_line_comments(
        " SELECT 'this is a \\' -- string ' --test, another `table` 123 \"back\" to 'dust'",
    );

    assert_eq!(2, result.len());
    assert_eq!([8,33], result[0]);
    assert_eq!([34,78], result[1]);
}

#[test]
#[ignore = "the single quote string parse was moved to the `parse_tokens` function"]
fn find_strings_and_line_comments_strings() {
    let result = find_strings_and_line_comments(
        " SELECT 'this is a \\' -- string ',\"this is also a \\\" -- string \"",
    );

    assert_eq!(2, result.len());
    assert_eq!([8, 33], result[0]);
    assert_eq!([34, 64], result[1]);
}
