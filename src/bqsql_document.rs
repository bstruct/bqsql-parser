use serde::Serialize;

use self::bqsql_interpreter::BqsqlInterpreter;

pub mod bqsql_delimiter;
pub mod bqsql_interpreter_query_from_test;
pub mod bqsql_interpreter_query_full_test;
pub mod bqsql_interpreter_query_select_test;
pub mod bqsql_interpreter_query;
pub mod bqsql_interpreter;
pub mod bqsql_keyword;
pub mod bqsql_operator;
pub mod bqsql_query_structure;
pub mod parser;
pub mod token_parser;

#[derive(Serialize, Clone)]
pub struct BqsqlDocument {
    pub items: Vec<BqsqlDocumentItem>,
}

#[derive(Serialize, Clone)]
pub struct BqsqlDocumentItem {
    pub item_type: BqsqlDocumentItemType,
    pub range: Option<[usize; 3]>,
    pub items: Vec<BqsqlDocumentItem>,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BqsqlDocumentItemType {
    Unknown,
    LineComment,
    Keyword,
    KeywordAs,

    String,
    Number,
    Operator,
    // TYPE,
    // FUNCTION,
    // METHOD,
    ParenthesesOpen,
    ParenthesesClose,
    SquareBracketsOpen,
    SquareBracketsClose,
    Comma,
    Semicolon,
    Dot,

    Alias,

    Query,

    QueryWith,
    QueryWithRecursive,
    //CTE stands for 'common table expressions'.
    //The name of the table in the WITH statement
    QueryCteName,

    QuerySelect,
    // QuerySelectAll,
    // QuerySelectDistinct,
    // QuerySelectAsStruct,
    // QuerySelectAsValue,

    QuerySelectListItem,
    // QuerySelectStar,
    // QuerySelectColumnName,

    QueryFrom,
    QueryWhere,
    QueryGroupBy,
    QueryRollup,
    QueryHaving,
    QueryQualify,
    QueryWindow,
    QueryOrderBy,
    QueryLimit,
    QueryOffset,
}

impl PartialEq<&BqsqlDocumentItemType> for BqsqlDocumentItemType {
    fn eq(&self, other: &&BqsqlDocumentItemType) -> bool {
        self.eq(other)
    }
}

impl BqsqlDocument {
    pub(crate) fn parse(bqsql: &str) -> BqsqlDocument {
        let mut bqsql_interpreter = BqsqlInterpreter::new(bqsql);

        BqsqlDocument {
            items: bqsql_interpreter.collect(),
        }
    }
}
