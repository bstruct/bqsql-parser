use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug)]
pub struct BqsqlDocument {
    key: String,
    value: String,
}



// #[derive(Debug, postgres_types::ToSql)]
// #[postgres(name = "pubsubmessage")]
// pub struct PostgresPubsubMessage {
//     //full list of types:
//     //https://github.com/sfackler/rust-postgres/blob/master/postgres-types/src/lib.rs
//     pub message_id: Option<i64>,
//     pub attributes: Option<Vec<PostgresPubsubMessageAttribute>>,
//     pub data: Option<Vec<u8>>,
//     pub ordering_key: Option<String>,
//     pub publish_time: std::time::SystemTime,
//     pub delivery_attempt: Option<i32>,
// }

// // #[cfg(feature = "derive")]
// impl<'a> PostgresPubsubMessage {
//     //build information from GCP pubsub
//     pub fn from_ps(
//         received_message: &'a crate::pubsub::api::ReceivedMessage,
//     ) -> Option<PostgresPubsubMessage> {
//         match &received_message.message {
//             Some(m) => Some(PostgresPubsubMessage {
//                 message_id: m.message_id.parse::<i64>().ok(),
//                 attributes: None,
//                 data: Some(m.data.to_vec()),
//                 ordering_key: get_optional_string(&m.ordering_key),
//                 publish_time: get_publish_time(&m.publish_time),
//                 delivery_attempt: Some(received_message.delivery_attempt),
//             }),
//             _ => None,
//         }
//     }
// }