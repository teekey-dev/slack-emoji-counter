use serde::{Deserialize, Serialize};
use super::Reaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "type")]
    pub message_type: String,
    pub sub_type: Option<String>,
    pub hidden: Option<bool>,
    pub channel: Option<String>,
    pub user: String,
    pub text: String,
    pub ts: String,
    pub deleted_ts: Option<String>,
    pub event_ts: Option<String>,
    pub is_starred: Option<bool>,
    pub pinned_to: Option<Vec::<String>>,
    pub edited: Option<MessageEdited>,
    pub reactions: Option<Vec::<Reaction>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEdited {
    pub user: String,
    pub ts: String
}
