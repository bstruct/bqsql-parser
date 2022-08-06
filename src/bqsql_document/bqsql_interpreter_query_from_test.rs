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
    //--- --- Query
    //--- --- --- QuerySelect
    //--- --- ---  ---Keyword
    //--- --- --- --- QuerySelectListItem
    //--- --- --- --- --- QuerySelectColumnName
    //--- --- --- QueryFrom
    //--- --- --- --- Keyword
    //--- --- --- --- QueryCteName
    //--- --- ParenthesesClose
    //--- --- LineComment

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
    assert_eq!(4, query.items.len());

    //--- QueryWith
    let query_with = &query.items[0];
    assert_eq!(BqsqlDocumentItemType::QueryWith, query_with.item_type);
    assert_eq!(None, query_with.range);
    assert_eq!(7, query_with.items.len());

    //--- --- Keyword
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_with.items[0].item_type
    );
    assert_eq!(Some([0, 0, 4]), query_with.items[0].range);
    assert_eq!(0, query_with.items[0].items.len());

    //--- --- QueryCteName
    assert_eq!(
        BqsqlDocumentItemType::QueryCteName,
        query_with.items[1].item_type
    );
    assert_eq!(Some([0, 5, 7]), query_with.items[1].range);
    assert_eq!(0, query_with.items[1].items.len());

    //--- --- KeywordAs
    assert_eq!(
        BqsqlDocumentItemType::KeywordAs,
        query_with.items[2].item_type
    );
    assert_eq!(Some([0, 8, 10]), query_with.items[2].range);
    assert_eq!(0, query_with.items[2].items.len());

    //--- --- ParenthesesOpen
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesOpen,
        query_with.items[3].item_type
    );
    assert_eq!(Some([0, 11, 12]), query_with.items[3].range);
    assert_eq!(0, query_with.items[3].items.len());

    //--- --- Query
    assert_eq!(BqsqlDocumentItemType::Query, query_with.items[4].item_type);
    assert_eq!(None, query_with.items[4].range);
    assert_eq!(2, query_with.items[4].items.len());

    let query_1_items = &query_with.items[4].items;
    //--- --- --- QuerySelect
    assert_eq!(
        BqsqlDocumentItemType::QuerySelect,
        query_1_items[0].item_type
    );
    assert_eq!(None, query_1_items[0].range);
    assert_eq!(2, query_1_items[0].items.len());

    let query_1_items_0_items = &query_1_items[0].items;
    //--- --- --- --- Keyword
    assert_eq!(
        BqsqlDocumentItemType::Keyword,
        query_1_items_0_items[0].item_type
    );
    assert_eq!(Some([0, 12, 18]), query_1_items_0_items[0].range);
    assert_eq!(0, query_1_items_0_items[0].items.len());

    //--- --- --- --- QuerySelectListItem
    // assert_eq!(
    //     BqsqlDocumentItemType::QuerySelectListItem,
    //     query_1_items_0_items[1].item_type
    // );
    // assert_eq!(None, query_1_items_0_items[1].range);
    // assert_eq!(1, query_1_items_0_items[1].items.len());

    //--- --- --- --- --- QuerySelectColumnName

    //--- --- --- QueryFrom
    assert_eq!(BqsqlDocumentItemType::QueryFrom, query_1_items[1].item_type);
    assert_eq!(None, query_1_items[1].range);
    assert_eq!(2, query_1_items[1].items.len());

    //--- --- --- --- Keyword
    //--- --- --- --- QueryCteName

    //--- --- ParenthesesClose
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesClose,
        query_with.items[5].item_type
    );
    assert_eq!(Some([0, 39, 40]), query_with.items[5].range);
    assert_eq!(0, query_with.items[5].items.len());

    //--- --- LineComment
    assert_eq!(
        BqsqlDocumentItemType::LineComment,
        query_with.items[6].item_type
    );
    assert_eq!(Some([0, 41, 50]), query_with.items[6].range);
    assert_eq!(0, query_with.items[6].items.len());

    //--- QuerySelect
    let query_select = &query.items[1];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(2, query_select.items.len());

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
    assert_eq!(BqsqlDocumentItemType::Semicolon, query.items[3].item_type);
    assert_eq!(Some([7, 17, 18]), query.items[3].range);
    assert_eq!(0, query.items[3].items.len());

    //
}
