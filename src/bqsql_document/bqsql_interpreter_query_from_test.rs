use crate::bqsql_document::{BqsqlDocument, BqsqlDocumentItemType};

#[test]
fn queries_with() {
    let document = BqsqlDocument::parse(
        r#"WITH q1 AS (SELECT SchoolID FROM Roster) #my_query
SELECT *
FROM
(WITH q2 AS (SELECT * FROM q1),  # q1 resolves to my_query
    q3 AS (SELECT * FROM q1),  # q1 resolves to my_query
    q1 AS (SELECT * FROM q1),  # q1 (in the query) resolves to my_query
    q4 AS (SELECT * FROM q1)   # q1 resolves to the WITH subquery on the previous line.
SELECT * FROM q1);             # q1 resolves to the third inner WITH subquery."#,
    );

    //
    //Query

    //--- QueryWith
    //--- --- Keyword
    //--- --- QueryCteName
    //--- --- KeywordAs
    //--- --- ParenthesesOpen
    //--- --- --- QuerySelect
    //--- --- ---  ---Keyword
    //--- --- --- --- QuerySelectListItem
    //--- --- --- --- --- QuerySelectColumnName
    //--- --- --- QueryFrom
    //--- --- --- --- Keyword
    //--- --- --- --- QueryCteName
    //--- --- ParenthesesClose

    //--- QuerySelect
    //--- --- Keyword
    //--- --- QuerySelectListItem
    //--- --- --- QuerySelectStar

    //--- QueryFrom
    //--- --- Keyword

    //--- --- ParenthesesOpen

    //--- --- QueryWith
    //--- --- --- Keyword
    //--- --- --- QueryCteName
    //--- --- --- KeywordAs
    //--- --- --- ParenthesesOpen
    //--- --- --- --- QuerySelect
    //--- --- --- --- --- Keyword
    //--- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- QueryFrom
    //--- --- --- --- --- Keyword
    //--- --- --- --- --- QueryCteName
    //--- --- --- ParenthesesClose
    //--- --- --- Comma

    //--- --- --- QueryCteName
    //--- --- --- KeywordAs
    //--- --- --- ParenthesesOpen
    //--- --- --- --- QuerySelect
    //--- --- --- ---  ---Keyword
    //--- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- QueryFrom
    //--- --- --- --- --- Keyword
    //--- --- --- --- --- QueryCteName
    //--- --- --- ParenthesesClose
    //--- --- --- Comma

    //--- --- --- QueryCteName
    //--- --- --- --- KeywordAs
    //--- --- --- --- ParenthesesOpen
    //--- --- --- --- --- QuerySelect
    //--- --- --- --- ---  ---Keyword
    //--- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- QueryCteName
    //--- --- --- --- ParenthesesClose
    //--- --- --- --- Comma

    //--- --- --- QueryCteName
    //--- --- --- --- KeywordAs
    //--- --- --- --- ParenthesesOpen
    //--- --- --- --- --- QuerySelect
    //--- --- --- --- ---  ---Keyword
    //--- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- QueryCteName
    //--- --- --- --- ParenthesesClose

    //--- --- QuerySelect
    //--- --- --- Keyword
    //--- --- --- QuerySelectListItem
    //--- --- --- --- QuerySelectStar
    //--- --- QueryFrom
    //--- --- --- Keyword
    //--- --- --- QueryCteName

    //--- --- ParenthesesClose
    //--- Semicolon
    //

    assert_eq!(1, document.items.len());

    //
    //Query

    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    // assert_eq!(4, query.items.len());

    //--- QueryWith
    let query_with = &query.items[0];
    assert_eq!(BqsqlDocumentItemType::QueryWith, query_with.item_type);
    assert_eq!(None, query_with.range);
    // assert_eq!(4, query_with.items.len());

    //--- --- Keyword
    assert_eq!(BqsqlDocumentItemType::Keyword, query_with.items[0].item_type);
    assert_eq!(None, query_with.items[0].range);
    assert_eq!(0, query_with.items[0].items.len());

    //--- --- QueryCteName
    //--- --- KeywordAs
    //--- --- ParenthesesOpen
    //--- --- --- QuerySelect
    //--- --- ---  ---Keyword
    //--- --- --- --- QuerySelectListItem
    //--- --- --- --- --- QuerySelectColumnName
    //--- --- --- QueryFrom
    //--- --- --- --- Keyword
    //--- --- --- --- QueryCteName
    //--- --- ParenthesesClose

    //--- QuerySelect
    //--- --- Keyword
    //--- --- QuerySelectListItem
    //--- --- --- QuerySelectStar

    //--- QueryFrom
    //--- --- Keyword

    //--- --- ParenthesesOpen

    //--- --- QueryWith
    //--- --- --- Keyword
    //--- --- --- QueryCteName
    //--- --- --- KeywordAs
    //--- --- --- ParenthesesOpen
    //--- --- --- --- QuerySelect
    //--- --- --- --- --- Keyword
    //--- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- QueryFrom
    //--- --- --- --- --- Keyword
    //--- --- --- --- --- QueryCteName
    //--- --- --- ParenthesesClose
    //--- --- --- Comma

    //--- --- --- QueryCteName
    //--- --- --- KeywordAs
    //--- --- --- ParenthesesOpen
    //--- --- --- --- QuerySelect
    //--- --- --- ---  ---Keyword
    //--- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- QueryFrom
    //--- --- --- --- --- Keyword
    //--- --- --- --- --- QueryCteName
    //--- --- --- ParenthesesClose
    //--- --- --- Comma

    //--- --- --- QueryCteName
    //--- --- --- --- KeywordAs
    //--- --- --- --- ParenthesesOpen
    //--- --- --- --- --- QuerySelect
    //--- --- --- --- ---  ---Keyword
    //--- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- QueryCteName
    //--- --- --- --- ParenthesesClose
    //--- --- --- --- Comma

    //--- --- --- QueryCteName
    //--- --- --- --- KeywordAs
    //--- --- --- --- ParenthesesOpen
    //--- --- --- --- --- QuerySelect
    //--- --- --- --- ---  ---Keyword
    //--- --- --- --- --- --- QuerySelectListItem
    //--- --- --- --- --- --- --- QuerySelectStar
    //--- --- --- --- --- QueryFrom
    //--- --- --- --- --- --- Keyword
    //--- --- --- --- --- --- QueryCteName
    //--- --- --- --- ParenthesesClose

    //--- --- QuerySelect
    //--- --- --- Keyword
    //--- --- --- QuerySelectListItem
    //--- --- --- --- QuerySelectStar
    //--- --- QueryFrom
    //--- --- --- Keyword
    //--- --- --- QueryCteName

    //--- --- ParenthesesClose
    //--- Semicolon
    //
}
