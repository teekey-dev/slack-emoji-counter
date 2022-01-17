use serde::{Deserialize, Serialize};
use super::{Message, ResponseMetadata};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConversationHistoryResponse {
    pub messages: Vec::<Message>,
    pub has_more: bool,
    pub pin_count: u32,
    pub response_metadata: ResponseMetadata
}

