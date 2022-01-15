use serde::{Deserialize, Serialize};
use super::{Topic, Purpose};

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub is_channel: bool,
    pub is_group: bool,
    pub is_im: bool,
    pub created: u32,
    pub creator: String,
    pub is_archived: bool,
    pub is_general: bool,
    pub unlinked: u8,
    pub name_normalized: String,
    pub is_shared: bool,
    pub is_ext_shared: bool,
    pub is_org_shared: bool,
    pub pending_shared: Vec::<bool>,
    pub is_pending_ext_shared: bool,
    pub is_member: bool,
    pub is_private: bool,
    pub is_mpim: bool,
    pub last_read: Option<String>,
    pub topic: Topic,
    pub purpose: Purpose,
    pub previous_names: Option<Vec::<String>>,
    pub num_members: u16,
    pub locale: Option<String>
}
