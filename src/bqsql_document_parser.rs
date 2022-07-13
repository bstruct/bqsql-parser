use crate::bqsql_document::*;

impl BqsqlDocument {
    //build information from GCP pubsub
    pub fn parse(bqsql: &str) -> BqsqlDocument {

        print!("{}", bqsql);

        BqsqlDocument {
            document_type: BqsqlDocumentType::UNKNOWN,
            // items: Vec::new(),
        }
    }

}