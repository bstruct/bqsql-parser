use crate::bqsql_document::{BqsqlDocumentItemType, BqsqlDocumentSuggestionType};

use super::{bqsql_interpreter::BqsqlInterpreter, BqsqlDocumentItem, BqsqlDocumentSuggestion};

impl BqsqlInterpreter<'_> {
    pub(crate) fn suggest(bqsql: &str, position: [usize; 2]) -> Vec<BqsqlDocumentSuggestion> {
        let document_items = &BqsqlInterpreter::new(bqsql).collect();
        let flat_items = &flat_document(document_items);
        let location_in_document = locate_in_document(flat_items, position);

        let mut suggestions = Vec::new();
        suggestions.append(&mut suggest_syntax(flat_items, location_in_document));
        // suggestions.append(&mut suggest_functions(flat_items, location_in_document));
        // suggestions.append(&mut suggest_columns(flat_items, location_in_document));

        suggestions
    }
}

fn suggest_syntax(flat_items: &Vec<&BqsqlDocumentItem>, location_in_document: (LocationInDocumentType, Option<usize>)) -> Vec<BqsqlDocumentSuggestion> {

    let mut suggestions : Vec<BqsqlDocumentSuggestion> = Vec::new();

    if location_in_document.0 == LocationInDocumentType::After {
        if let Some(index) = location_in_document.1{

            if flat_items[index].item_type == BqsqlDocumentItemType::Keyword{


                
            } 
            
        }
    }

    suggestions
}

#[test]
fn suggest_nothing() {
    let suggestions = BqsqlInterpreter::suggest("SELECT * ", [0, 3]);

    assert_eq!(0, suggestions.len());
}

#[test]
fn suggest_except_from() {
    let suggestions = BqsqlInterpreter::suggest("SELECT * ", [0, 9]);

    assert_eq!(2, suggestions.len());

    //EXCEPT
    assert_eq!(
        BqsqlDocumentSuggestionType::Syntax,
        suggestions[0].suggestion_type
    );
    assert_eq!("EXCEPT", suggestions[0].name);
    assert_eq!("EXCEPT(${0:some_column}),", suggestions[0].snippet);

    //FROM
    assert_eq!(
        BqsqlDocumentSuggestionType::Syntax,
        suggestions[1].suggestion_type
    );
    assert_eq!("FROM", suggestions[1].name);
    assert_eq!("FROM ${0:some_table}", suggestions[1].snippet);
}

fn flat_document<'a>(document_items: &'a Vec<BqsqlDocumentItem>) -> Vec<&'a BqsqlDocumentItem> {
    let mut flat_items: Vec<&'a BqsqlDocumentItem> = Vec::new();

    for item in document_items {
        flat_items.push(item);
        if item.items.len() > 0 {
            flat_items.append(&mut flat_document(&item.items));
        }
    }

    flat_items
}

#[test]
fn flat_document_simple() {
    let document_items = &BqsqlInterpreter::new("SELECT 1,2,3,4,5 FROM t").collect();

    let flat_items = flat_document(document_items);
    assert_eq!(20, flat_items.len())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LocationInDocumentType {
    None,
    Middle,
    After,
}

fn locate_in_document<'a>(
    flat_items: &'a Vec<&'a BqsqlDocumentItem>,
    position: [usize; 2],
) -> (LocationInDocumentType, Option<usize>) {
    let mut previous_index: Option<usize> = None;

    if flat_items.len() > 0 {
        // let mut iter = flat_items.iter();
        let mut index: usize = 0;

        while index < flat_items.len() {
            let item = flat_items[index];

            if let Some(range) = item.range {
                if range[0] == position[0] {
                    if range[1] < position[1] && range[2] > position[1] {
                        return (LocationInDocumentType::Middle, Some(index));
                    } else if range[1] >= position[1] {
                        return (LocationInDocumentType::After, previous_index);
                    }
                } else if range[0] > position[0] {
                    return (LocationInDocumentType::After, previous_index);
                }
                previous_index = Some(index);
            }
            index += 1;
        }
        if previous_index.is_some() {
            return (LocationInDocumentType::After, previous_index);
        }
    }

    (LocationInDocumentType::None, None)
}

#[test]
fn locate_in_document_beggining() {
    let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
    let flat_items = &flat_document(document_items);

    let locate = locate_in_document(flat_items, [0, 0]);
    assert_eq!(LocationInDocumentType::None, locate.0);
    assert!(locate.1.is_none());
}

#[test]
fn locate_in_document_middle_1() {
    let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
    let flat_items = &flat_document(document_items);

    let locate = locate_in_document(flat_items, [0, 1]);
    assert_eq!(LocationInDocumentType::Middle, locate.0);
    assert!(locate.1.is_some());
    let item = flat_items[locate.1.unwrap()];
    assert_eq!(BqsqlDocumentItemType::Keyword, item.item_type);
    assert_eq!(Some([0, 0, 6]), item.range);
}

#[test]
fn locate_in_document_middle_3() {
    let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
    let flat_items = &flat_document(document_items);

    let locate = locate_in_document(flat_items, [0, 3]);
    assert_eq!(LocationInDocumentType::Middle, locate.0);
    assert!(locate.1.is_some());
    let item = flat_items[locate.1.unwrap()];
    assert_eq!(BqsqlDocumentItemType::Keyword, item.item_type);
    assert_eq!(Some([0, 0, 6]), item.range);
}

#[test]
fn locate_in_document_after_6() {
    let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
    let flat_items = &flat_document(document_items);

    let locate = locate_in_document(flat_items, [0, 6]);
    assert_eq!(LocationInDocumentType::After, locate.0);
    assert!(locate.1.is_some());
    let item = flat_items[locate.1.unwrap()];
    assert_eq!(BqsqlDocumentItemType::Keyword, item.item_type);
    assert_eq!(Some([0, 0, 6]), item.range);
}

#[test]
fn locate_in_document_after_7() {
    let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
    let flat_items = &flat_document(document_items);

    let locate = locate_in_document(flat_items, [0, 7]);
    assert_eq!(LocationInDocumentType::After, locate.0);
    assert!(locate.1.is_some());
    let item = flat_items[locate.1.unwrap()];
    assert_eq!(BqsqlDocumentItemType::Keyword, item.item_type);
    assert_eq!(Some([0, 0, 6]), item.range);
}

#[test]
fn locate_in_document_after_8() {
    let document_items = &BqsqlInterpreter::new("SELECT * ").collect();
    let flat_items = &flat_document(document_items);

    let locate = locate_in_document(flat_items, [0, 8]);
    assert_eq!(LocationInDocumentType::After, locate.0);
    assert!(locate.1.is_some());
    let item = flat_items[locate.1.unwrap()];
    assert_eq!(BqsqlDocumentItemType::Operator, item.item_type);
    assert_eq!(Some([0, 7, 8]), item.range);
}
