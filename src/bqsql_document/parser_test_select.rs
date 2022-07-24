use crate::bqsql_document::{BqsqlDocument, BqsqlDocumentItemType};

#[test]
fn empty_string() {
    let document = BqsqlDocument::parse("");

    assert_eq!(0, document.items.len());
}

#[test]
fn comment_only() {
    let document = BqsqlDocument::parse("--super comment");

    assert_eq!(0, document.items.len());
}

#[test]
fn comment_with_select_in_text() {
    let document = BqsqlDocument::parse("--super comment that includes a query SELECT 2+2");

    assert_eq!(0, document.items.len());
}

#[test]
fn space_comment_only() {
    let document = BqsqlDocument::parse("    --super comment");

    assert_eq!(0, document.items.len());
}

#[test]
fn tiny_query() {
    let document = BqsqlDocument::parse("SELECT 2+2");

    assert_eq!(1, document.items.len());

    assert_eq!(BqsqlDocumentItemType::Query, document.items[1].item_type);
    assert_eq!(None, document.items[1].range);
    assert_eq!(1, document.items[1].items.len());

    assert_eq!(
        BqsqlDocumentItemType::QuerySelect,
        document.items[1].items[1].item_type
    );
    // assert_eq!(0, document.items[0].from.column);
    // assert_eq!(0, document.items[0].from.line);
    // assert_eq!(0, document.items[0].from.index);
    // assert_eq!(10, document.items[0].to.column);
    // assert_eq!(0, document.items[0].to.line);
    // assert_eq!(10, document.items[0].to.index);
}

#[test]
fn tiny_query_second_line() {
    let document = BqsqlDocument::parse("\nSELECT 2+2");

    //
    //Query
    //--- QuerySelect
    //--- --- KEYWORD
    //--- --- QUERY_SELECT_LIST_ITEM
    //--- --- --- NUMBER
    //--- --- --- OPERATOR
    //--- --- --- NUMBER
    //

    assert_eq!(1, document.items.len());

    //
    //QUERY
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(1, query.items.len());

    //--- QUERY_SELECT
    let query_select = &document.items[0].items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(2, query_select.items.len());

    //--- --- KEYWORD
    let k_0 = &query_select.items[0];
    assert_eq!(BqsqlDocumentItemType::Keyword, k_0.item_type);
    assert_eq!(Some([1, 0, 6]), k_0.range);
    assert_eq!(0, k_0.items.len());

    //--- --- QUERY_SELECT_LIST_ITEM
    let query_list_item_0 = &query_select.items[1];
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_list_item_0.item_type
    );
    assert_eq!(None, query_list_item_0.range);
    assert_eq!(3, query_list_item_0.items.len());

    let query_list_item_0_items = &query_list_item_0.items;
    assert_eq!(
        BqsqlDocumentItemType::Number,
        query_list_item_0_items[0].item_type
    );
    assert_eq!(Some([1, 7, 8]), query_list_item_0_items[0].range);
    assert_eq!(0, query_list_item_0_items[0].items.len());

    assert_eq!(
        BqsqlDocumentItemType::OPERATOR,
        query_list_item_0_items[1].item_type
    );
    assert_eq!(Some([1, 8, 9]), query_list_item_0_items[1].range);
    assert_eq!(0, query_list_item_0_items[1].items.len());

    assert_eq!(
        BqsqlDocumentItemType::Number,
        query_list_item_0_items[2].item_type
    );
    assert_eq!(Some([1, 9, 10]), query_list_item_0_items[2].range);
    assert_eq!(0, query_list_item_0_items[2].items.len());

}

#[test]
fn comment_and_tiny_query() {
    let document = BqsqlDocument::parse("--super comment\nSELECT 2+2");

    assert_eq!(1, document.items.len());
    // assert_eq!(BqsqlDocumentItemType::COMMENT, document.items[0].item_type);
    // assert_eq!(0, document.items[0].from.column);
    // assert_eq!(0, document.items[0].from.line);
    // assert_eq!(0, document.items[0].from.index);
    // assert_eq!(14, document.items[0].to.column);
    // assert_eq!(0, document.items[0].to.line);
    // assert_eq!(14, document.items[0].to.index);
    // assert_eq!(BqsqlDocumentItemType::QUERY, document.items[1].item_type);
    // assert_eq!(0, document.items[1].from.column);
    // assert_eq!(1, document.items[1].from.line);
    // assert_eq!(16, document.items[1].from.index);
    // assert_eq!(0, document.items[1].to.column);
    // assert_eq!(1, document.items[1].to.line);
    // assert_eq!(16, document.items[1].to.index);
}

#[test]
fn select_select_as_struct_query() {
    let document =
        BqsqlDocument::parse("SELECT (SELECT AS STRUCT 2+2 AS asas, 'ASDASD' AS qweqwe) AS XXX");

    //
    //QUERY
    //--- QUERY_SELECT
    //--- --- KEYWORD
    //--- --- QUERY_SELECT_LIST_ITEM
    //--- --- --- PARENTHESES_OPEN
    //--- --- --- QUERY
    //--- --- --- --- QUERY_SELECT_AS_STRUCT
    //--- --- --- --- --- KEYWORD
    //--- --- --- --- --- KEYWORD
    //--- --- --- --- --- KEYWORD
    //--- --- --- --- --- QUERY_SELECT_LIST_ITEM
    //--- --- --- --- --- --- NUMBER
    //--- --- --- --- --- --- OPERATOR
    //--- --- --- --- --- --- NUMBER
    //--- --- --- --- --- --- AS_ALIAS
    //--- --- --- --- --- --- ALIAS
    //--- --- --- --- --- --- Comma
    //--- --- --- --- --- QUERY_SELECT_LIST_ITEM
    //--- --- --- --- --- --- STRING
    //--- --- --- --- --- --- AS_ALIAS
    //--- --- --- --- --- --- ALIAS
    //--- --- --- PARENTHESES_CLOSE
    //--- --- --- AS_ALIAS
    //--- --- --- ALIAS
    //

    assert_eq!(1, document.items.len());

    //
    //QUERY
    let query = &document.items[0];
    assert_eq!(BqsqlDocumentItemType::Query, query.item_type);
    assert_eq!(None, query.range);
    assert_eq!(1, query.items.len());

    //--- QUERY_SELECT
    let query_select = &document.items[0].items[0];
    assert_eq!(BqsqlDocumentItemType::QuerySelect, query_select.item_type);
    assert_eq!(None, query_select.range);
    assert_eq!(2, query_select.items.len());

    //--- --- KEYWORD
    let k_0 = &query_select.items[0];
    assert_eq!(BqsqlDocumentItemType::Keyword, k_0.item_type);
    assert_eq!(Some([0, 0, 6]), k_0.range);
    assert_eq!(0, k_0.items.len());

    //--- --- QUERY_SELECT_LIST_ITEM
    let query_list_item_0 = &query_select.items[1];
    assert_eq!(
        BqsqlDocumentItemType::QuerySelectListItem,
        query_list_item_0.item_type
    );
    assert_eq!(None, query_list_item_0.range);
    assert_eq!(5, query_list_item_0.items.len());

    //--- --- --- PARENTHESES_OPEN
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesOpen,
        query_list_item_0.items[0].item_type
    );
    assert_eq!(None, query_list_item_0.items[0].range);
    assert_eq!(Some([0, 7, 8]), query_list_item_0.items[0].range);

    //--- --- --- QUERY
    assert_eq!(
        BqsqlDocumentItemType::Query,
        query_list_item_0.items[1].item_type
    );
    assert_eq!(None, query_list_item_0.items[1].range);
    assert_eq!(1, query_list_item_0.items[1].items.len());

    //--- --- --- --- QUERY_SELECT_AS_STRUCT
    //--- --- --- --- --- KEYWORD
    //--- --- --- --- --- KEYWORD
    //--- --- --- --- --- KEYWORD
    //--- --- --- --- --- QUERY_SELECT_SELECT_LIST_ITEM
    //--- --- --- --- --- --- NUMBER
    //--- --- --- --- --- --- OPERATOR
    //--- --- --- --- --- --- NUMBER
    //--- --- --- --- --- --- AS_ALIAS
    //--- --- --- --- --- --- ALIAS
    //--- --- --- --- --- QUERY_SELECT_SELECT_LIST_ITEM
    //--- --- --- --- --- --- STRING
    //--- --- --- --- --- --- AS_ALIAS
    //--- --- --- --- --- --- ALIAS

    //--- --- --- PARENTHESES_CLOSE
    assert_eq!(
        BqsqlDocumentItemType::ParenthesesClose,
        query_list_item_0.items[2].item_type
    );
    assert_eq!(Some([1, 2, 3]), query_list_item_0.items[2].range);
    assert_eq!(0, query_list_item_0.items[2].items.len());

    //--- --- --- AS_ALIAS
    assert_eq!(
        BqsqlDocumentItemType::AsAlias,
        query_list_item_0.items[2].item_type
    );
    assert_eq!(Some([1, 2, 3]), query_list_item_0.items[2].range);
    assert_eq!(0, query_list_item_0.items[2].items.len());

    //--- --- --- ALIAS
    assert_eq!(
        BqsqlDocumentItemType::Alias,
        query_list_item_0.items[3].item_type
    );
    assert_eq!(Some([1, 2, 3]), query_list_item_0.items[3].range);
    assert_eq!(0, query_list_item_0.items[3].items.len());

    //
    //
}
