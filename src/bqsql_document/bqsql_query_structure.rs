// query_statement:
//  query_expr

// query_expr:
// [ WITH [ RECURSIVE ] { non_recursive_cte | recursive_cte }[, ...] ]
// { select | ( query_expr ) | set_operation }
// [ ORDER BY expression [{ ASC | DESC }] [, ...] ]
// [ LIMIT count [ OFFSET skip_rows ] ]

// select:
// SELECT
//     [ { ALL | DISTINCT } ]
//     [ AS { STRUCT | VALUE } ]
//     select_list
// [ FROM from_clause[, ...] ]
// [ WHERE bool_expression ]
// [ GROUP BY { expression [, ...] | ROLLUP ( expression [, ...] ) } ]
// [ HAVING bool_expression ]
// [ QUALIFY bool_expression ]
// [ WINDOW window_clause ]

#[derive(Debug, Clone, Copy)]
pub(crate) enum BqsqlQueryStructure {
    With = 1,
    Select = 2,
    From = 3,
    Where = 4,
    GroupBy = 5,
    Rollup = 6,
    Having = 7,
    Qualify = 8,
    Window = 9,
    OrderBy = 10,
    Limit = 11,
    Offset = 12,
}
