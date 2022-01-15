use serde::{Deserialize, Serialize};
use super::Channel;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConversationListResponse {
    pub channels: Vec::<Channel>
}
