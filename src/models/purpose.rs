use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Purpose {
    pub value: String,
    pub creator: String,
    pub last_set: u32
}
