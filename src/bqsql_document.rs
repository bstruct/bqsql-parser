use serde::Serialize;

use self::bqsql_interpreter::BqsqlInterpreter;

pub mod bqsql_delimiter;
pub mod bqsql_interpreter;
pub mod bqsql_interpreter_query;
#[cfg(test)]
pub mod bqsql_interpreter_query_from_test;
#[cfg(test)]
pub mod bqsql_interpreter_query_full_test;
#[cfg(test)]
pub mod bqsql_interpreter_query_select_test;
pub mod bqsql_interpreter_suggest;
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
    pub keyword: Option<BqsqlKeyword>
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BqsqlDocumentItemType {
    Unknown,
    LineComment,
    Keyword,

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
    //CTE stands for 'common table expressions'.
    //The name of the table in the WITH statement
    QueryCteName,

    QuerySelect,
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

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub enum BqsqlKeyword {
    All,
    As,
    Distinct,
    From,
    Recursive,
    Select,
    Struct,
    Value,
    Where,
    With,
    Group,
    By,
    Rollup,
    Having,
    Qualify,
    Window,
    Order,
    Limit,
    Offset,
}

#[derive(Serialize, Clone)]
pub struct BqsqlDocumentSuggestion {
    pub suggestion_type: BqsqlDocumentSuggestionType,
    pub name: String,
    pub snippet: String,
}

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BqsqlDocumentSuggestionType {
    Syntax,
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

    pub(crate) fn suggest(bqsql: &str, position: [usize; 2]) -> Vec<BqsqlDocumentSuggestion> {
        BqsqlInterpreter::suggest(bqsql, position)
    }
}
