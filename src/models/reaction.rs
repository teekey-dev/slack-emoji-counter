use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
    pub name: String,
    pub count: u16,
    pub users: Vec::<String>
}

